package common

import "fmt"

// Set represents a generic set.
type Set[T comparable] struct {
	elements map[T]struct{}
}

// NewSet creates a new empty set.
func NewSet[T comparable]() *Set[T] {
	return &Set[T]{elements: make(map[T]struct{})}
}

// Add inserts an element into the set.
func (s *Set[T]) Add(value T) {
	s.elements[value] = struct{}{}
}

// Remove deletes an element from the set.
func (s *Set[T]) Remove(value T) {
	delete(s.elements, value)
}

// Contains checks if the set contains a given element.
func (s *Set[T]) Contains(value T) bool {
	_, exists := s.elements[value]
	return exists
}

// Size returns the number of elements in the set.
func (s *Set[T]) Size() int {
	return len(s.elements)
}

// Clear removes all elements from the set.
func (s *Set[T]) Clear() {
	s.elements = make(map[T]struct{})
}

// ToSlice returns a slice of all elements in the set.
func (s *Set[T]) ToSlice() []T {
	result := make([]T, 0, len(s.elements))
	for key := range s.elements {
		result = append(result, key)
	}
	return result
}

// Union combines two sets and returns a new set.
func (s *Set[T]) Union(other *Set[T]) *Set[T] {
	result := NewSet[T]()
	for key := range s.elements {
		result.Add(key)
	}
	for key := range other.elements {
		result.Add(key)
	}
	return result
}

// Intersection returns a new set with elements common to both sets.
func (s *Set[T]) Intersection(other *Set[T]) *Set[T] {
	result := NewSet[T]()
	for key := range s.elements {
		if other.Contains(key) {
			result.Add(key)
		}
	}
	return result
}

// Difference returns a new set with elements in the current set but not in the other.
func (s *Set[T]) Difference(other *Set[T]) *Set[T] {
	result := NewSet[T]()
	for key := range s.elements {
		if !other.Contains(key) {
			result.Add(key)
		}
	}
	return result
}

// IsSubset checks if the current set is a subset of another.
func (s *Set[T]) IsSubset(other *Set[T]) bool {
	for key := range s.elements {
		if !other.Contains(key) {
			return false
		}
	}
	return true
}

// String returns a string representation of the set.
func (s *Set[T]) String() string {
	return fmt.Sprintf("%v", s.ToSlice())
}
