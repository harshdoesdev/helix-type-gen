import { createTypedClient } from './helix-client'
import HelixDBClient from 'helix-ts';

const client = createTypedClient(new HelixDBClient("http://localhost:6969"));

async function main() {
  try {
    const users = await client.GetUserFriends({ user_id: 12345 });
    console.log('Fetched Users:', users);
  } catch (error) {
    console.error('Error:', error);
  }
}

main()
