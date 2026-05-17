@echo off
cd /d C:\Users\mario\Documents\PE2-CLI
echo Current state:
git log --oneline -5
echo.
echo Files in index:
git ls-files --cached do_all.ps1 cleanup.ps1 .sisyphus 2>&1
echo.
echo Working tree:
git status --porcelain 2>&1
echo.
echo ===== Cleaning up =====
echo Step 1: Save current README from latest commit
git show HEAD:README.md > C:\Users\mario\AppData\Local\Temp\readme_fixed.md 2>&1
echo.
echo Step 2: Hard reset to da65219 (last good commit on remote)
git reset --hard da65219 2>&1
echo.
echo Step 3: Restore the fixed README
copy /y C:\Users\mario\AppData\Local\Temp\readme_fixed.md README.md >nul
echo.
echo Step 4: Remove temp files from disk
if exist "do_all.ps1" del /f /q "do_all.ps1"
if exist "cleanup.ps1" del /f /q "cleanup.ps1"
if exist ".sisyphus" rmdir /s /q ".sisyphus"
echo.
echo Step 5: Stage everything
git add -A 2>&1
echo.
echo Step 6: Commit
git commit -m "fix: update README badges, remove .sisyphus" 2>&1
echo.
echo Step 7: Push
git push origin main 2>&1
echo.
echo Done.
