import { ref } from 'vue';
import mqtt from 'mqtt';
import { useAvatarStore } from '@/store';

export function useMqtt() {
  const mqttData = ref(null);
  const avatar = useAvatarStore();
  const client = mqtt.connect(import.meta.env.VITE_MQTT_BROKER);

  client.on('connect', () => {
    client.subscribe('blockrock/iot/light', (err) => {
      if (!err) console.log('Subscribed to light topic');
    });
  });

  client.on('message', (topic, message) => {
    mqttData.value = JSON.parse(message.toString());
    avatar.authentications += 1;
    avatar.skills.lights = Math.min(avatar.skills.lights + 5, 100);
  });

  const publish = (topic, message) => {
    client.publish(topic, message);
  };

  return { mqttData, publish };
}
