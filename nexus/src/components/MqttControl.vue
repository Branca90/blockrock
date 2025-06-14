<template>
  <v-card>
    <v-card-title>Controllo IoT</v-card-title>
    <v-card-text>
      <v-btn color="teal" @click="sendCommand">Accendi Luce</v-btn>
      <p>Stato: {{ mqttData?.status || 'Sconnesso' }}</p>
    </v-card-text>
  </v-card>
</template>
<script>
import { useMqtt } from '@/composables/useMqtt';
import { useTron } from '@/composables/useTron';
import { useToast } from 'vue-toastification';

export default {
  setup() {
    const { mqttData, publish } = useMqtt();
    const { createTransaction } = useTron();
    const toast = useToast();

    const sendCommand = async () => {
      try {
        publish('blockrock/iot/light', JSON.stringify({ command: 'on' }));
        await createTransaction('Accendi luce', 1);
        toast.success('Luce accesa e transazione registrata!');
      } catch (error) {
        toast.error('Errore: ' + error.message);
      }
    };

    return { mqttData, sendCommand };
  },
};
</script>
