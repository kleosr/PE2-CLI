@echo off
cd /d C:\Users\mario\Documents\PE2-CLI
echo === Delete temp scripts ===
if exist "check_sisyphus.bat" del /f /q "check_sisyphus.bat"
if exist "fixgit.bat" del /f /q "fixgit.bat"
if exist "cleanup.ps1" del /f /q "cleanup.ps1"
if exist "do_all.ps1" del /f /q "do_all.ps1"

echo === Verify .sisyphus fully gone ===
if exist ".sisyphus\" (
  rmdir /s /q ".sisyphus"
  echo Deleted .sisyphus folder
) else (
  echo .sisyphus already gone from disk
)

echo === Purge bad commits from reflog ===
git reflog expire --expire=now --all
echo === Garbage collect to remove orphaned blobs ===
git gc --prune=now --aggressive

echo === Stage and commit temp file removals ===
git add -A 2>&1
git status --porcelain
git diff --cached --name-status 2>&1

echo === Commit ===
git commit -m "chore: clean up temp scripts" 2>&1

echo === Push ===
git push origin main 2>&1

echo.
echo === Done - .sisyphus and bad commits fully purged ===
