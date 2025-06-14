<template>
  <v-app :theme="theme">
    <v-navigation-drawer v-model="drawer" app>
      <v-list>
        <v-list-item to="/" title="Dashboard" prepend-icon="mdi-home" />
        <v-list-item to="/settings" title="Impostazioni" prepend-icon="mdi-cog" />
      </v-list>
      <template v-slot:append>
        <AvatarCard mini />
      </template>
    </v-navigation-drawer>
    <v-app-bar app color="deep-purple" dark>
      <v-app-bar-nav-icon @click="drawer = !drawer" />
      <v-img src="/nexus-logo.png" max-height="40" max-width="40" class="mr-2" />
      <v-toolbar-title>Nexus Dashboard</v-toolbar-title>
      <v-spacer />
      <v-btn icon @click="toggleTheme">
        <v-icon>{{ theme === 'dark' ? 'mdi-white-balance-sunny' : 'mdi-moon-waxing-crescent' }}</v-icon>
      </v-btn>
    </v-app-bar>
    <v-main>
      <slot />
    </v-main>
  </v-app>
</template>
<script>
import { ref } from 'vue';
import AvatarCard from '@/components/AvatarCard.vue';

export default {
  components: { AvatarCard },
  setup() {
    const drawer = ref(true);
    const theme = ref('dark');

    const toggleTheme = () => {
      theme.value = theme.value === 'dark' ? 'light' : 'dark';
    };

    return { drawer, theme, toggleTheme };
  },
};
</script>
