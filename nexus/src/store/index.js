import { defineStore } from 'pinia';

export const useAvatarStore = defineStore('avatar', {
  state: () => ({
    level: 1,
    authentications: 42,
    skills: { lights: 80, gps: 60, eeg: 20 },
  }),
});
