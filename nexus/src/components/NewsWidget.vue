<template>
  <v-card>
    <v-card-title>News</v-card-title>
    <v-card-text>
      <p>{{ news?.title || 'Nessuna notizia' }}</p>
    </v-card-text>
  </v-card>
</template>
<script>
import axios from 'axios';

export default {
  data() {
    return {
      news: null,
    };
  },
  async mounted() {
    try {
      const response = await axios.get('https://zion-core:3000/news'); // Sostituisci con la tua API
      this.news = response.data[0];
    } catch (error) {
      console.error('Errore news:', error);
    }
  },
};
</script>
