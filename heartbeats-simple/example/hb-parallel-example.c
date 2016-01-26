/**
 *  Example of using heartbeat in parallel.
 */
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <sys/time.h>
#ifdef __MACH__
#include <mach/clock.h>
#include <mach/mach.h>
#endif
#include <inttypes.h>
#include <unistd.h>
#include <pthread.h>
#include "heartbeat.h"

#define NTHREADS 10
#define ITERATIONS 10
// window size should be >= nthreads (larger is more accurate)
#define WINDOW_SIZE 10

// Callback function for when a window is complete (window buffer is full)
void window_complete(const heartbeat_context* hb) {
  // dummy function
}

// get time from the system in nanoseconds
static inline uint64_t get_time() {
  struct timespec ts;
#ifdef __MACH__
  // OS X does not have clock_gettime, use clock_get_time
  clock_serv_t cclock;
  mach_timespec_t mts;
  host_get_clock_service(mach_host_self(), CALENDAR_CLOCK, &cclock);
  clock_get_time(cclock, &mts);
  mach_port_deallocate(mach_task_self(), cclock);
  ts.tv_sec = mts.tv_sec;
  ts.tv_nsec = mts.tv_nsec;
#else
  clock_gettime(CLOCK_REALTIME, &ts);
#endif
  return ts.tv_sec * 1000000000 + ts.tv_nsec;
}

typedef struct worker_params {
  heartbeat_context* hb;
  unsigned int worker_id;
  unsigned int iterations;
  uint64_t* last_end_time;
  volatile int* lock;
} worker_params;

void* worker(void* arg) {
  worker_params* params = (worker_params*) arg;
  int i;
  uint64_t start_time, end_time;

  for (i = 0; i < params->iterations; i++) {
    start_time = get_time();
    usleep(1000000);
    end_time = get_time();
    while (__sync_lock_test_and_set(params->lock, 1)) {
      while (*params->lock);
    }
    start_time = *params->last_end_time > start_time ? *params->last_end_time : start_time;
    end_time = start_time > end_time ? start_time : end_time;
    heartbeat(params->hb, params->worker_id * 100 + i, 1, start_time, end_time);
    *params->last_end_time = end_time;
    __sync_lock_release(params->lock);
  }

  return NULL;
}

int main(void) {
  pthread_t threads[NTHREADS];
  worker_params params[NTHREADS];
  uint64_t i;
  int fd = fileno(stdout);

  // Alternatively, a window buffer can be allocated on the stack with a
  // statically sized array - just don't let it go out of scope before
  // the heartbeat!
  heartbeat_record* window_buffer = malloc(WINDOW_SIZE * sizeof(heartbeat_record));

  // initialize heartbeat
  heartbeat_context hb;
  heartbeat_init(&hb, WINDOW_SIZE, window_buffer, fd, &window_complete);
  hb_log_header(fd);

  uint64_t last_end_time = 0;
  volatile int lock;
  for(i = 0; i < NTHREADS; i++) {
    params[i] = (worker_params) {&hb, i, ITERATIONS, &last_end_time, &lock};
    pthread_create(&threads[i], NULL, worker, &params[i]);
  }

  for(i = 0; i < NTHREADS; i++) {
    pthread_join(threads[i], NULL);
  }

  // cleanup memory
  free(window_buffer);

  return 0;
}
