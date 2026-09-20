# Website deployment

`lumis.sh` is built from this Vite app by the `lumis-website` Vercel project.
The project uses `website/` as its root directory and `main` as its production
branch. `vercel.json` defines the build, skips deploys without website changes,
and redirects the former `/docs` entry to `docs.lumis.sh`.

The GitHub Pages workflow and `public/CNAME` remain until the DNS cutover is
verified. After that, they can be removed in a separate change.
