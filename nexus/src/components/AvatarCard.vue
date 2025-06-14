<template>
  <v-card :class="{ 'avatar-card': !mini, 'avatar-mini': mini }">
    <v-card-title v-if="!mini">Nexus Avatar</v-card-title>
    <v-card-text>
      <Vue3Lottie
        :animationLink="animationSrc"
        :height="mini ? 50 : 200"
        :width="mini ? 50 : 200"
        loop
        autoPlay
      />
      <div v-if="!mini">
        <p>Livello: {{ avatar.level }} - Neoterra Pioneer</p>
        <p>Autenticazioni: {{ avatar.authentications }}</p>
        <p>Forza: Luci ({{ avatar.skills.lights }}%)</p>
        <p>Agilità: GPS ({{ avatar.skills.gps }}%)</p>
        <p>Intelligenza: EEG ({{ avatar.skills.eeg }}%)</p>
      </div>
    </v-card-text>
  </v-card>
</template>
<script>
import { useAvatarStore } from '@/store';
import { ref, watch } from 'vue';

export default {
  props: { mini: Boolean },
  setup() {
    const avatar = useAvatarStore();
    const animationSrc = ref('/animations/avatar.json');

    watch(() => avatar.authentications, (newValue) => {
      animationSrc.value = newValue > 100
        ? '/animations/avatar-level-up.json'
        : '/animations/avatar.json';
      avatar.level = Math.floor(newValue / 100) + 1;
    });

    return { avatar, animationSrc };
  },
};
</script>
<style>
.avatar-card { border: 2px solid #FF5722; background: #1a202c; color: white; }
.avatar-mini { border: none; background: transparent; }
</style>
