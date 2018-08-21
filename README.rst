D'accord
========

Domain Availability CheCker d'accORD


Cli tools which are originally introduced as ``domainfinder`` in the
`Go: Building Web Applications`_ Book. This is a project inspired by it,
and written in Rust.


.. _`Go: Building Web Applications`: https://github.com/PacktPublishing/Go-Building-Web-Applications


.. code:: zsh

  % cd src/sprinkle
  % echo "hoi" | cargo run
  gohoi

  % cd src/domainify
  % echo "What's up?" | cargo run
  whats-up.com

  % cd src/coolify
  % echo "dog" | cargo run
  doog

  % echo "cat" | sprinkle | coolify | domainify
  lets-caat.net



License
-------


.. code:: text

   D'accord
   Copyright 2018 Yasuhiro Asaka

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
