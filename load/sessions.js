import http from 'k6/http';

export const options = {
  vus: 20,
  duration: '2m'
};

export default function () {
  const url = `${__ENV.API_URL ?? 'http://localhost:3000'}/sessions`;
  http.post(
    url,
    JSON.stringify({
      goal_id: 'goal-demo',
      duration_minutes: 60
    }),
    {
      headers: { 'Content-Type': 'application/json' }
    }
  );
}
