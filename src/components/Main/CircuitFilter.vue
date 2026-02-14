<template>
    <v-card class="mb-4" :color="cardColor">
        <v-card-title class="d-flex align-center text-body-2">
            <v-icon start>mdi-filter-variant</v-icon>
            Filtrage & Tri
            <v-spacer></v-spacer>
            <v-btn 
                @click="resetFilters" 
                variant="text" 
                size="small" 
                color="blue"
                prepend-icon="mdi-filter-variant-remove"
                :disabled="isResetDisabled || isResetting"
            >
                Réinitialiser
            </v-btn>
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
import { watch, reactive, computed, ref } from 'vue';
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
const isResetting = ref(false);

const isResetDisabled = computed(() => {
    // Check if filters are at their default values
    const isFiltersDefault = 
        filters.nom === '' &&
        filters.villeId === null &&
        filters.traceurId === null &&
        filters.distance[0] === props.filterData.minDistance &&
        filters.distance[1] === props.filterData.maxDistance &&
        filters.denivele[0] === props.filterData.minDenivele &&
        filters.denivele[1] === props.filterData.maxDenivele;
    
    // Check if sort is at its default value
    const isSortDefault = 
        sort.by === 'circuitId' &&
        sort.order === 'asc';
        
    return isFiltersDefault && isSortDefault;
});

watch(filters, (newFilters) => {
    emit('update:modelValue', newFilters);
});

watch(sort, (newSort) => {
    emit('update:sortValue', newSort);
});

const resetFilters = () => {
    isResetting.value = true;
    filters.nom = '';
    filters.villeId = null;
    filters.traceurId = null;
    filters.distance = [props.filterData.minDistance, props.filterData.maxDistance];
    filters.denivele = [props.filterData.minDenivele, props.filterData.maxDenivele];
    
    sort.by = 'circuitId';
    sort.order = 'asc';

    // The button will be disabled by isResetDisabled once reactivity cycles, 
    // but isResetting ensures it's immediate and handles the click event completion.
    setTimeout(() => {
        isResetting.value = false;
    }, 100);
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
