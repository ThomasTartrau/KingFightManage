<script setup lang="ts">
import { CircleUserRound, ShieldHalf, Users } from 'lucide-vue-next'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
</script>

<template>
  <div class="grid gap-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
    <Card class="rounded-2xl">
      <CardHeader>
        <div class="flex justify-between">
          <CardTitle> Active users </CardTitle>
          <Users color="pink" :size="48" />
        </div>
      </CardHeader>
      <CardContent>
        <div class="text-2xl">
          100
        </div>
      </CardContent>
    </Card>

    <Card class="rounded-2xl">
      <CardHeader>
        <div class="flex justify-between">
          <CardTitle> Active staffs </CardTitle>
          <CircleUserRound color="pink" :size="48" />
        </div>
      </CardHeader>
      <CardContent>
        <div class="text-2xl">
          2
        </div>
      </CardContent>
    </Card>

    <Card class="rounded-2xl md:col-span-2 lg:col-span-1">
      <CardHeader>
        <div class="flex justify-between">
          <CardTitle> Role </CardTitle>
          <ShieldHalf color="pink" :size="48" />
        </div>
      </CardHeader>
      <CardContent>
        <div class="text-2xl">
          Moderateur
        </div>
      </CardContent>
    </Card>
  </div>
</template>
