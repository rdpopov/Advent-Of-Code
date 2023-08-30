package main

import "testing"
// Here are a few more examples:
//     10players1618 points: high score is 8317
//     13players7999 points: high score is 146373
//     17players1104 points: high score is 2764
//     21players6111 points: high score is 54718
//     30players5807 points: high score is 37305
// What is the winning Elf's score?

func Test30players5807(t *testing.T){
    // var lst List
    got := Part1(6111,21)
    want := 37305
    if got != want {
        t.Errorf("got %d, wanted %d", got, want)
    }
}

func Test21players6111(t *testing.T){
    // var lst List
    got := Part1(6111,21)
    want := 54718
    if got != want {
        t.Errorf("got %d, wanted %d", got, want)
    }
}

func Test17players1104(t *testing.T){
    // var lst List
    got := Part1(1104,17)
    want := 2764
    if got != want {
        t.Errorf("got %d, wanted %d", got, want)
    }
}

func Test10players1618(t *testing.T){
    // var lst List
    got := Part1(1618,10)
    want := 8317
    if got != want {
        t.Errorf("got %d, wanted %d", got, want)
    }
}

func Test13players7999(t *testing.T){
    // var lst List
    got := Part1(7999,13)
    want := 146373
    if got != want {
        t.Errorf("got %d, wanted %d", got, want)
    }
}
