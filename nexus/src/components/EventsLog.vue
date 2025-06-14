<template>
  <v-card>
    <v-card-title>Log Eventi</v-card-title>
    <v-card-text>
      <v-list>
        <v-list-item v-for="event in events" :key="event.id">
          {{ event.message }} - {{ event.timestamp }}
        </v-list-item>
      </v-list>
    </v-card-text>
  </v-card>
</template>
<script>
import { ref } from 'vue';
import axios from 'axios';

export default {
  setup() {
    const events = ref([]);
    const fetchEvents = async () => {
      try {
        const response = await axios.get('https://zion-core:3000/events'); // Sostituisci con la tua API
        events.value = response.data;
      } catch (error) {
        console.error('Errore eventi:', error);
      }
    };
    fetchEvents();
    return { events };
  },
};
</script>
