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

use std::collections::VecDeque;

fn lookbehind_matches(lookbehind: &VecDeque<Option<char>>, string: &str) -> bool {
  string.chars().map(|x| Some(x)).eq(lookbehind.iter().map(|x| *x).skip(lookbehind.len() - string.len()))
}

pub fn strip_comments(single_line_markers: &'static [&'static str],
                      multi_line_markers: &'static [(&'static str, &'static str)],
                      quote_marks: &'static [&'static str]) -> impl Fn(&str) -> String {

  let single_line_max: usize = single_line_markers.iter().map(|x| x.len()).max().unwrap_or(0);
  let multi_line_max: usize = multi_line_markers.iter().map(|(x, y)| x.len().max(y.len())).max().unwrap_or(0);
  let quote_max: usize = quote_marks.iter().map(|x| x.len()).max().unwrap_or(0);

  let lookbehind_width: usize = single_line_max.max(multi_line_max).max(quote_max);

  move |text: &str| {
    let mut ret = String::with_capacity(text.len());

    let mut lookbehind = VecDeque::new();
    lookbehind.resize(lookbehind_width, None);
    let mut in_comment = false;
    // the expected comment end marker, or None for EOL
    let mut comment_end = None;
    let mut quote_end = None;
    for c in text.chars() {
      lookbehind.pop_front();
      lookbehind.push_back(Some(c));

      if quote_end.is_none() && !in_comment {
        ret.push(c);
        if let Some(mark) = single_line_markers.into_iter().find(|m| lookbehind_matches(&lookbehind, m)) {
          in_comment = true;
          comment_end = None;
          ret.truncate(ret.len() - mark.len());
          continue;
        }
        if let Some((_, end)) = multi_line_markers.into_iter().find(|(s, e)| lookbehind_matches(&lookbehind, s)) {
          in_comment = true;
          comment_end = Some(end);
          ret.truncate(ret.len() - end.len());
          continue;
        }
        if let Some(quote) = quote_marks.into_iter().find(|m| lookbehind_matches(&lookbehind, m)) {
          quote_end = Some(quote);
          continue;
        }
      }
      else if let Some(end) = quote_end {
        ret.push(c);
        if lookbehind_matches(&lookbehind, end) {
          quote_end = None;
          continue;
        }
      }
      else if in_comment {
        if let Some(end) = comment_end {
          if lookbehind_matches(&lookbehind, end) {
            in_comment = false;
            continue;
          }
        }
        else {
          if [Some('\n'), Some('\r')].contains(lookbehind.back().unwrap()) {
            in_comment = false;
            continue;
          }
        }
      }
    }
    ret
  }
}

pub fn c_like() -> impl Fn(&str) -> String { strip_comments(&["//"], &[("/*", "*/")], &["\""]) }
pub fn python_like() -> impl Fn(&str) -> String { strip_comments(&["#"], &[], &["\"", "'"]) }
