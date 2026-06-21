| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `.\target\release\day6_one_sync.exe .\solutions\year_2015\src\input\day6.txt` | 23.5 ± 1.7 | 21.6 | 27.6 | 1.00 |
| `.\target\release\day6_one_thread.exe .\solutions\year_2015\src\input\day6.txt` | 79.3 ± 1.4 | 77.6 | 82.1 | 3.38 ± 0.25 |
| `.\target\release\day6_one_tokio.exe .\solutions\year_2015\src\input\day6.txt` | 24.7 ± 1.4 | 21.5 | 26.9 | 1.05 ± 0.09 |

`hyperfine --export-markdown bench_day6.md -w 3 -r 10 '.\target\release\day6_one_sync.exe ".\solutions\year_2015\src\input\day6.txt"' '.\target\release\day6_one_thread.exe ".\solutions\year_2015\src\input\day6.txt"' '.\target\release\day6_one_tokio.exe ".\solutions\year_2015\src\input\day6.txt"'`