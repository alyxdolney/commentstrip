<?php
/* Copyright (C) 2024 by Alyx Dolney

   Permission to use, copy, modify, and/or distribute this software for any
   purpose with or without fee is hereby granted.

   THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
   WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
   MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
   SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
   OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
   CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE. */

function strip_comments(array $single_line_markers, array $multi_line_markers, array $quote_marks) {
  return function (string $text) use ($single_line_markers, $multi_line_markers, $quote_marks) {
    $ret = "";

    $in_comment = false;
    // the expected comment end marker, or null for EOL
    $comment_end = null;
    $quote_end = null;
    foreach (str_split($text) as $i => $c) {
      $start = substr($text, 0, $i);
      $rest = substr($text, $i);
      if ($quote_end === null && !$in_comment) {
        $ret .= $c;
        if (@array_filter($single_line_markers, fn($m) => str_starts_with($rest, $m))[0]) {
          $in_comment = true;
          $comment_end = null;
          $ret = substr($ret, 0, -1);
          continue;
        }
        if ($marker = @array_filter($multi_line_markers, fn($m) => str_starts_with($rest, $m[0]))[0]) {
          $in_comment = true;
          $comment_end = $marker[1];
          $ret = substr($ret, 0, -1);
          continue;
        }
        if ($marker = @array_filter($quote_marks, fn($m) => str_starts_with($rest, $m))[0]) {
          $quote_end = $marker;
          continue;
        }
      }
      else if ($quote_end !== null) {
        $ret .= $c;
        if (str_ends_with($start, $quote_end) && !str_ends_with($start, '\\'.$quote_end)) {
          $quote_end = null;
          continue;
        }
      }
      else if ($in_comment) {
        if ($comment_end !== null) {
          echo $comment_end.'/'.$start."\n";
          if (str_ends_with($start, $comment_end)) {
            $ret .= $c;
            $in_comment = false;
            continue;
          }
        }
        else {
          if (in_array($rest[0], ['\r', '\n'], true)) {
            $ret .= $c;
            $in_comment = false;
            continue;
          }
        }
      }
    }
    return $ret;
  };
}

$c_like = strip_comments(["//"], [["/*", "*/"]], ["\""]);
$python_like = strip_comments(["#"], [], ["\"", "'"]);
