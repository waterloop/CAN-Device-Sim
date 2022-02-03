// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
const fs =  require('fs/promises');
const path = require('path');
const moment = require('moment');

export default async  function handler(req, res) {
  const { method, body } = req;
  const date = new Date();
  const jsonBody = JSON.parse(body);
  switch (method) {
  case 'POST': {
    const config = {
      ...jsonBody,
      info: {
        ...jsonBody.info,
        dateCreated: date.toDateString()
      }
    }
    const fileName = `${config.info.subteam}-${config.info.author}-${config.info.dateCreated}.json`;
    try {
      // Try to write the file to the system. If the file already exists, then fail.
      // There is very little chance that the file should exist since the name is based on the dateCreated string.
      await fs.writeFile(path.join(__dirname, '../../../../../configs', fileName), JSON.stringify(config), { encoding: "utf-8", flag: 'ax' });
      res.status(200);
    } catch (err) {
      console.error(err);
      res.status(500).end('There was an error Creating the config. Please try again');
    };
    break;
  }
  default:
    res.setHeader('Allow', ['POST']);
    res.status(405).end(`Method ${method} Not Allowed`);
  }
}
