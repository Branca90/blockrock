<template>
  <Dashboard>
    <v-container fluid>
      <v-row>
        <v-col cols="12">
          <v-card>
            <v-card-title>Impostazioni</v-card-title>
            <v-card-text>
              <p>Configura le impostazioni di Nexus qui.</p>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </Dashboard>
</template>
<script>
import Dashboard from '@/layouts/Dashboard.vue';

export default {
  components: { Dashboard },
};
</script>
