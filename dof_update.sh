#!/usr/bin/env bash

git checkout main
git pull || exit
git checkout dof


git merge --no-ff --no-commit main
for f in README.md dof_update.sh; do
git reset HEAD $f
git checkout -- $f
done


for f in *; do
  if [[ $(git ls-files $f) != "" && $f != "dof_update.sh" && $f != README.md ]]; then
    git checkout main -- $f
  fi
done


for f in *; do
  if [[ $(git ls-files $f) != "" && $f != "dof_update.sh" && $f != README.md ]]; then
    echo $f
    for name in cfg src local layer alias var seq fakekeys chords aliasenvcond overrides; do
      find $f -type f -print0 | xargs -0 sed -i "s/\bdef$name/dof$name/g"
    done
  fi
done

git add .
git commit -m "merged main at $(git rev-parse main)"
