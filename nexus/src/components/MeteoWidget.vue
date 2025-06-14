<template>
  <v-card>
    <v-card-title>Meteo</v-card-title>
    <v-card-text>
      <p>Temperatura: {{ weather?.temp || 'N/A' }}°C</p>
      <p>Condizioni: {{ weather?.description || 'N/A' }}</p>
    </v-card-text>
  </v-card>
</template>
<script>
import axios from 'axios';

export default {
  data() {
    return {
      weather: null,
    };
  },
  async mounted() {
    try {
      const response = await axios.get(
        `https://api.openweathermap.org/data/2.5/weather?q=Milano&appid=${import.meta.env.VITE_OPENWEATHERMAP_API_KEY}&units=metric`
      );
      this.weather = {
        temp: response.data.main.temp,
        description: response.data.weather[0].description,
      };
    } catch (error) {
      console.error('Errore meteo:', error);
    }
  },
};
</script>
