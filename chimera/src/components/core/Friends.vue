<template>
  <div>
    <v-navigation-drawer v-model="friendsDrawer" app class="black overflow-visible" right width="200" static style="overflow: visible">
      <v-row class="mx-0 flex-column fill-height" justify="center">
        <v-col>
          <v-list class="py-0" color="transparent" two-line>
            <auth />
            <template>
              <v-divider class="mb-5" />
              <span class="title ml-1">{{selectedGroup.instanceName}}</span>
              <v-menu v-for="(friend, i) in groupFriends" :key="i" :offset-x="$vuetify.breakpoint.mdAndUp" :offset-y="$vuetify.breakpoint.smAndDown" :left="$vuetify.breakpoint.mdAndUp" :min-width="$vuetify.breakpoint.smAndDown ? '100%' : undefined" attach :max-width="$vuetify.breakpoint.mdAndUp ? 200 : undefined" style="max-height: 0; max-width: 0;">
                <template v-slot:activator="{ attrs, on }">
                  <v-list-item link v-bind="attrs" class="mx-n3" v-on="on">
                    <v-list-item-action class="justify-center">
                      <v-progress-circular :color="friend.online ? 'green' : 'red'" size="48" :value="friend.value" :rotate="friend.start">
                        <v-badge overlap :content="friend.notifications" :value="friend.notifications" color="green" offset-x="23" offset-y="23">
                            <v-list-item-avatar v-if="friend.info">
                              <v-img :src="friend.info.avatar" />
                            </v-list-item-avatar>
                        </v-badge>
                      </v-progress-circular>
                    </v-list-item-action>
                    <v-list-item-content>
                      <v-list-item-title v-text="friend.name" />
                      <v-list-item-subtitle v-if="friend.info">
                        {{ friend.info.name }}
                      </v-list-item-subtitle>
                      <v-list-item-subtitle v-else>
                        {{ friend.online ? 'Online' : 'Offline' }}
                      </v-list-item-subtitle>
                    </v-list-item-content>
                  </v-list-item>
                </template>
                <v-list :color="$vuetify.breakpoint.mdAndUp ? 'black' : 'grey darken-4'">
                  <v-list-item @click="whisperFriend(friend.info.name)">
                    <v-list-item-action>
                      <v-btn icon>
                        <v-icon>mdi-chat</v-icon>
                      </v-btn>
                    </v-list-item-action>
                    <v-list-item-content>
                      <v-list-item-title>Whisper</v-list-item-title>
                      <v-list-item-title>{{friend.agentAddress}}s</v-list-item-title>
                    </v-list-item-content>
                  </v-list-item>
                  <!-- <v-list-item @click="whisperFriend">
                    <v-list-item-action>
                      <v-btn icon>
                        <v-icon>mdi-message-video</v-icon>
                      </v-btn>
                    </v-list-item-action>
                    <v-list-item-content>
                      <v-list-item-title>Connect</v-list-item-title>
                    </v-list-item-content>
                  </v-list-item> -->
                  <!-- <v-list-item @click="inviteFriend">
                    <v-list-item-action>
                      <v-btn icon>
                        <v-icon>mdi-account-plus</v-icon>
                      </v-btn>
                    </v-list-item-action>
                    <v-list-item-content>
                      <v-list-item-title>Invite</v-list-item-title>
                    </v-list-item-content>
                  </v-list-item>
                  <v-list-item @click="removeFriend">
                    <v-list-item-action>
                      <v-btn icon>
                        <v-icon>mdi-account-remove</v-icon>
                      </v-btn>
                    </v-list-item-action>
                    <v-list-item-content>
                      <v-list-item-title>Remove</v-list-item-title>
                    </v-list-item-content>
                  </v-list-item> -->
                </v-list>
              </v-menu>
            </template>
          </v-list>
        </v-col>
      </v-row>
    </v-navigation-drawer>
    <core-fab />
    <v-dialog v-model="chatDialog" max-width="700px">
      <v-card flat>
        <v-toolbar dark>
          <v-toolbar-title class="display-1">Whisper</v-toolbar-title>
          <v-spacer></v-spacer>
        </v-toolbar>
        <v-content>
         Chat window
        </v-content>
        <v-spacer></v-spacer>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="action darken-1" text @click="chatDialog = false">
            Close
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script>
import { mapGetters, mapMutations, mapState } from 'vuex'
export default {
  name: 'CoreFriends',
  components: {
    Auth: () => import('@/components/friends/Auth'),
    CoreFab: () => import('./Fab')
  },
  data () {
    return {
      chatDialog: false
    }
  },
  computed: {
    ...mapGetters('friends', ['online', 'friends']),
    ...mapState('auth', ['loggedIn']),
    ...mapState('friends', ['drawer', 'selectedGroup']),
    groupFriends () {
      return this.friends(this.selectedGroup.instanceId)
    },
    friendsDrawer: {
      get () {
        return this.drawer
      },
      set (val) {
        this.setDrawer(val)
      }
    }
  },
  methods: {
    ...mapMutations('friends', ['setDrawer']),
    menu (event) {
      //
    },
    removeFriend () {
      //
    },
    whisperFriend (name) {
      //  hdk::send(member_id, json!({"msg_type": msg_type, "id": &conversation_address}).to_string(), 10000.into())?;
    },
    inviteFriend () {
      //
    }
  }
}
</script>
