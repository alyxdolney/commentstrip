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

function strip_comments(single_line_markers, multi_line_markers, quote_marks) {
  return function (text) {
    let ret = "";

    let in_comment = false;
    // the expected comment end marker, or null for EOL
    let comment_end = null;
    let quote_end = null;
    for (i in text) {
      const start = text.slice(0, i);
      const rest = text.slice(i);
      const c = text[i];
      if (quote_end === null && !in_comment) {
        ret += c;
        let marker;
        if (single_line_markers.find(m => rest.startsWith(m))) {
          in_comment = true;
          comment_end = null;
          ret = ret.slice(0, -1);
          continue;
        }
        if (marker = multi_line_markers.find(m => rest.startsWith(m[0]))) {
          in_comment = true;
          comment_end = marker[1];
          ret = ret.slice(0, -1);
          continue;
        }
        if (marker = quote_marks.find(m => rest.startsWith(m))) {
          quote_end = marker;
          continue;
        }
      }
      else if (quote_end !== null) {
        ret += c;
        if (start.endsWith(quote_end) && !start.endsWith('\\' + quote_end)) {
          quote_end = null;
          continue;
        }
      }
      else if (in_comment) {
        if (comment_end !== null) {
          if (start.endsWith(comment_end)) {
            ret += c;
            in_comment = false;
            continue;
          }
        }
        else {
          if (['\r', '\n'].includes(rest[0])) {
            ret += c;
            in_comment = false;
            continue;
          }
        }
      }
    }
    return ret;
  }
}

const c_like = strip_comments(["//"], [["/*", "*/"]], ["\""]);
const python_like = strip_comments(["#"], [], ["\"", "'"]);
