#!/usr/bin/bash
for f in *; do
  if [[ $(git ls-files $f) != "" && $f != "dof_update.sh" && $f != README.md ]]; then
    git checkout main -- $f
    echo $f
    for name in cfg src local layer alias var seq fakekeys chords aliasenvcond; do
      find $f -type f -print0 | xargs -0 sed -i "s/\bdef$name/dof$name/g"
    done
  fi
done
