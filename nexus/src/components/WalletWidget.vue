<template>
  <v-card>
    <v-card-title>Wallet Tron</v-card-title>
    <v-card-text>
      <p>Saldo: {{ balance }} TRX</p>
    </v-card-text>
  </v-card>
</template>
<script>
import { useTron } from '@/composables/useTron';

export default {
  setup() {
    const { balance } = useTron();
    return { balance };
  },
};
</script>
