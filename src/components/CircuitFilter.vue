<template>
    <v-card class="mb-4" :color="cardColor">
        <v-card-title class="d-flex align-center text-body-2">
            <v-icon start>mdi-filter-variant</v-icon>
            Filtrage & Tri
            <v-spacer></v-spacer>
            <v-btn @click="resetFilters" variant="text" size="small">Réinitialiser</v-btn>
        </v-card-title>
        <v-card-text class="pt-2">
            <v-row dense align="center">
                <!-- Name -->
                <v-col cols="12" md="2">
                    <v-text-field
                        v-model="filters.nom"
                        label="Nom du circuit"
                        clearable
                        hide-details
                        density="compact"
                    >
                        <template v-slot:append-inner>
                            <v-btn icon :color="sort.by === 'nom' ? 'primary' : ''" @click="setSort('nom')" size="x-small" variant="text">
                                <v-icon>{{ getSortIcon('nom') }}</v-icon>
                            </v-btn>
                        </template>
                    </v-text-field>
                </v-col>

                 <!-- City -->
                <v-col cols="12" md="2">
                    <v-autocomplete
                        v-model="filters.villeId"
                        :items="filterData.villes"
                        item-title="nom"
                        item-value="id"
                        label="Ville de départ"
                        clearable
                        hide-details
                        density="compact"
                        autocomplete="off"
                    >
                        <template v-slot:append-inner>
                            <v-btn icon :color="sort.by === 'villeDepart' ? 'primary' : ''" @click="setSort('villeDepart')" size="x-small" variant="text">
                                <v-icon>{{ getSortIcon('villeDepart') }}</v-icon>
                            </v-btn>
                        </template>
                    </v-autocomplete>
                </v-col>

                <!-- Distance -->
                <v-col cols="12" md="3">
                    <v-range-slider
                        v-model="filters.distance"
                        :min="filterData.minDistance"
                        :max="filterData.maxDistance"
                        :step="1"
                        label="Distance"
                        thumb-label="always"
                        hide-details
                        density="compact"
                        class="align-center"
                    >
                        <template v-slot:prepend>
                             <v-btn icon :color="sort.by === 'distanceKm' ? 'primary' : ''" @click="setSort('distanceKm')" size="x-small" variant="text">
                                <v-icon>{{ getSortIcon('distanceKm') }}</v-icon>
                            </v-btn>
                        </template>
                        <template v-slot:thumb-label="{ modelValue }">
                            {{ modelValue }} km
                        </template>
                    </v-range-slider>
                </v-col>

                <!-- Elevation -->
                <v-col cols="12" md="3">
                     <v-range-slider
                        v-model="filters.denivele"
                        :min="filterData.minDenivele"
                        :max="filterData.maxDenivele"
                        :step="10"
                        label="Dénivelé"
                        thumb-label="always"
                        hide-details
                        density="compact"
                        class="align-center"
                    >
                        <template v-slot:prepend>
                             <v-btn icon :color="sort.by === 'deniveleM' ? 'primary' : ''" @click="setSort('deniveleM')" size="x-small" variant="text">
                                <v-icon>{{ getSortIcon('deniveleM') }}</v-icon>
                            </v-btn>
                        </template>
                        <template v-slot:thumb-label="{ modelValue }">
                            {{ modelValue }} m
                        </template>
                    </v-range-slider>
                </v-col>

                <!-- Tracer -->
                <v-col cols="12" md="2" class="pl-8">
                    <v-autocomplete
                        v-model="filters.traceurId"
                        :items="filterData.traceurs"
                        item-title="nom"
                        item-value="id"
                        label="Traceur"
                        clearable
                        hide-details
                        density="compact"
                        autocomplete="off"
                    >
                        <template v-slot:append-inner>
                            <v-btn icon :color="sort.by === 'traceur' ? 'primary' : ''" @click="setSort('traceur')" size="x-small" variant="text">
                                <v-icon>{{ getSortIcon('traceur') }}</v-icon>
                            </v-btn>
                        </template>
                    </v-autocomplete>
                </v-col>
            </v-row>
        </v-card-text>
    </v-card>
</template>

<script setup>
import { watch, reactive, computed } from 'vue';
import { useTheme } from 'vuetify';

const theme = useTheme();

const cardColor = computed(() => {
  return theme.global.current.value.dark ? 'grey-darken-4' : 'grey-lighten-5';
});

const props = defineProps({
    filterData: {
        type: Object,
        required: true
    },
    modelValue: {
        type: Object,
        required: true
    },
    sortValue: {
        type: Object,
        required: true
    }
});

const emit = defineEmits(['update:modelValue', 'update:sortValue']);

const filters = reactive({ ...props.modelValue });
const sort = reactive({ ...props.sortValue });

watch(filters, (newFilters) => {
    emit('update:modelValue', newFilters);
});

watch(sort, (newSort) => {
    emit('update:sortValue', newSort);
});

const resetFilters = () => {
    filters.nom = '';
    filters.villeId = null;
    filters.traceurId = null;
    filters.distance = [props.filterData.minDistance, props.filterData.maxDistance];
    filters.denivele = [props.filterData.minDenivele, props.filterData.maxDenivele];
};

const setSort = (field) => {
    if (sort.by === field) {
        sort.order = sort.order === 'asc' ? 'desc' : 'asc';
    } else {
        sort.by = field;
        sort.order = 'asc';
    }
};

const getSortIcon = (field) => {
    if (sort.by !== field) {
        return 'mdi-sort';
    }
    if (sort.order === 'asc') {
        return 'mdi-sort-ascending';
    }
    return 'mdi-sort-descending';
};

</script>

<style scoped>
.v-card-title {
    padding-top: 8px;
    padding-bottom: 4px;
}
.v-range-slider {
    margin-top: 28px;
}

:deep(.v-slider-thumb__label) {
  min-width: 60px;
  text-align: center;
}
</style>
