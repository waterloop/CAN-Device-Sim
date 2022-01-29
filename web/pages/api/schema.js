// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
const fs =  require('fs/promises');
const path = require('path');

export default async  function handler(req, res) {
  const { method } = req;

  switch (method) {
  case 'GET': {
    const schema = await fs.readFile(path.join(__dirname, '../../../../../schema/config.json'), { encoding: "utf-8" });
    res.status(200).json(JSON.parse(schema));
    break;
  }
  default:
    res.setHeader('Allow', ['GET']);
    res.status(405).end(`Method ${method} Not Allowed`);
  }
}
