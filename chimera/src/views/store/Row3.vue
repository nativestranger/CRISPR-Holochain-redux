<template>
  <v-row>
    <v-col cols="12" md="6">
      <v-row>
        <v-col cols="12">
          <part :value="leftTop" />
        </v-col>
        <v-col cols="12">
          <part :value="leftBottom" />
        </v-col>
      </v-row>
    </v-col>
    <v-col cols="12" md="6">
      <holochain-part :value="right" tall prominent />
    </v-col>
  </v-row>
</template>

<script>
export default {
  components: {
    HolochainPart: () => import('@/components/HolochainPart')
  },
  props: {
    value: {
      type: Array,
      default: () => ([])
    }
  },
  computed: {
    right () {
      return this.value[2]
    },
    leftBottom () {
      return this.value[1]
    },
    leftTop () {
      return this.value[0]
    }
  }
}
</script>
