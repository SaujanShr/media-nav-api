import express, { Request, Response } from 'express';
import { ANIME, META } from './consumet/src';

const app  = express();
const port = Number(process.env.PORT ?? 4000);
const mal  = new META.Myanimelist(new ANIME.AnimePahe());

// ── Routes ─────────────────────────────────────────────────────────────────────

// GET /anime/:malId/episodes
app.get('/anime/:malId/episodes', async (req: Request, res: Response) => {
  try {
    const info = await mal.fetchAnimeInfo(req.params.malId);
    res.json({ episodes: info.episodes ?? [] });
  } catch (e) {
    res.status(500).json({ error: String(e) });
  }
});

// GET /episode/sources?episodeId=gogoanime$...
app.get('/episode/sources', async (req: Request, res: Response) => {
  const episodeId = String(req.query.episodeId ?? '');
  if (!episodeId) {
    res.status(400).json({ error: 'episodeId is required' });
    return;
  }
  try {
    const sources = await mal.fetchEpisodeSources(episodeId);
    res.json(sources);
  } catch (e) {
    res.status(500).json({ error: String(e) });
  }
});

// ── Start ──────────────────────────────────────────────────────────────────────

app.listen(port, () => console.log(`consumet server listening on :${port}`));

