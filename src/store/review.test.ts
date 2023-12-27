import { setActivePinia, createPinia } from 'pinia'
import { useReviewStore } from './review'
import { it, describe, expect, beforeEach } from 'vitest';

describe('testing review store', () => {
  beforeEach(() => {
    // creates a fresh pinia and makes it active
    // so it's automatically picked up by any useStore() call
    // without having to pass it to it: `useStore(pinia)`
    setActivePinia(createPinia())
  })

  it('increments', () => {
    const counter = useReviewStore()
    expect(counter.content).toBe(0)
    counter.increment()
    expect(counter.content).toBe(1)
  })

  // it('increments by amount', () => {
  //   const counter = useReviewStore()
  //   counter.increment(10)
  //   expect(counter.content).toBe(10)
  // })
})
