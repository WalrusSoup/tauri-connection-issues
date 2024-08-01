<script setup lang="ts">
  import { Chat } from '@pubnub/chat';
  const startReallyLongRequest = () => {
    fetch('http://127.0.0.1:8888/uwu', {
      connectTimeout: 280000
    } as RequestInit)
      .then(response => response.text())
      .then(json => {
        // this won't happen
        console.log(json);
      })
      .catch(err => {
        // this won't happen
        console.error(err);
      })
  }


  Chat.init({
        publishKey: import.meta.env.VITE_PUBNUB_PUB_KEY,
        subscribeKey: import.meta.env.VITE_PUBNUB_SUB_KEY,
        userId: "3817",
        storeUserActivityTimestamps: true,
        restore: true,
        // these have to be here, though, im not *WHAT* they are fixing. Restore does not work unless these are present?
        // rather, when these are present, it seems like it will throw a timeout but *not* throw a PNBadRequestCategory error
        // NO IDEA WHY!
        // presenceTimeout: 50,
        // subscribeRequestTimeout: 50,

      }).then((chat) => {
        chat.sdk.addListener({
          status: (statusEvent) => {
            console.log('[chat] Status', statusEvent);
            if (statusEvent.category === 'PNBadRequestCategory') {
              console.error('Bad category detected');
            } else if (statusEvent.operation === 'PNUnsubscribeOperation') {
              console.error('UNSUBSCRIBED FROM KNOWN CHANNEL, WAS THIS AN ERROR?', statusEvent.affectedChannels);
            }
          },
        });
        chat = chat;
        // console.log('[chat] joining channels');
        chat?.currentUser
          .getMemberships()
          .then(({ memberships }) => {
            memberships.forEach((membership) => {
              console.log('[chat] joining channel', membership.channel.id);
              
              membership.channel.join((msg) => {
                console.log(msg);
              }).then(() => {
                console.log('CONNECTED');
              });
            });
          });
        })
</script>

<template>
  <div class="container">
    <h1>Check network tab</h1>
    <button @click="startReallyLongRequest">Start Really Long Request</button>
    <p>From the docs:
      https://www.pubnub.com/docs/general/setup/connection-management#subscribe-connections
    </p>
    <p style="max-width: 500px; margin: 0 auto;">
      A subscribe request creates a long-lived connection. The PubNub SDK makes an HTTP request to the PubNub Network which includes a list of channels to receive messages. When any client sends a message to one of those channels, PubNub responds to that subscribe request with the message content and a 200 status (success). The PubNub SDK will continue to issue new subscribe requests with each successful response to the previous subscribe request.

If no messages are sent on any of those channels after 280 seconds (the subscribe long poll expiration), the PubNub Network responds with no messages and a 200 status. Just like the message sent/received scenario above, the PubNub SDK submits a subsequent subscribe request to continue to listen for messages on those channels.

This subscribe request/response cycle uses the same connection for the entirety of that client's session (the life of the client's PubNub instance).
    </p>
  </div>
</template>