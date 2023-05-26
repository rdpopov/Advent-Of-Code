#lang racket

(define inp-file "./input")
(define (task-input fname) (map string->number (file->lines fname)))

(define (parse-line l)
  (string-split l #rx":|-| " )
  )
(print (parse-line "2-9 c: ccccccccc"))
