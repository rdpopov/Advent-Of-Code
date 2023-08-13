package main

import (
	"testing"
)


func TestNewPersonPositiveAge(t *testing.T) {
	a := add(1,2);
	if 3 != a {
		t.Errorf("Expected 3");
	}
}

func TestFail(t *testing.T) {
	a := add(1,2);
	if 3 == a {
		t.Errorf("Expected 3");
	}
}
