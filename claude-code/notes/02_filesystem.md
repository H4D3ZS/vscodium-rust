# 2. Filesystem Layout

/home/claude/               <- SCRATCHPAD (resets between tasks)
                               Claude writes work-in-progress here.
                               Like /tmp — do not rely on it persisting.

/mnt/user-data/
  uploads/                  <- YOUR UPLOADED FILES (read-only for Claude)
                               When you attach a file in chat, it lands here.
  outputs/                  <- DELIVERABLES (Claude copies final files here)
                               Files here become downloadable links for you.

/mnt/skills/public/         <- SKILL GUIDES (read-only)
                               Markdown docs teaching Claude how to produce
                               specific file types: docx, pdf, pptx, xlsx...
                               Claude reads these before creating those files.

/mnt/transcripts/           <- Conversation logs (read-only)

## Security note on /mnt paths
These are bind-mounted into the container from the host.
/mnt/user-data/uploads is intentionally read-only — Claude can read
your files but cannot modify the originals.
