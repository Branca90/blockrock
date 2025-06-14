<template>
  <v-card>
    <v-card-title>NAS</v-card-title>
    <v-card-text>
      <p>Spazio disponibile: {{ nas?.freeSpace || 'N/A' }}</p>
    </v-card-text>
  </v-card>
</template>
<script>
import axios from 'axios';

export default {
  data() {
    return {
      nas: null,
    };
  },
  async mounted() {
    try {
      const response = await axios.get('https://zion-core:3000/nas'); // Sostituisci con la tua API
      this.nas = response.data;
    } catch (error) {
      console.error('Errore NAS:', error);
    }
  },
};
</script>
