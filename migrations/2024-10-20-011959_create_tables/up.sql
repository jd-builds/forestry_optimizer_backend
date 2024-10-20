CREATE TABLE "blocks" (
    "id" UUID NOT NULL,
    "external_block_id" INTEGER NOT NULL,
    "net_merch" INTEGER NOT NULL,
    "volume_per_tree" FLOAT(53) NOT NULL,
    "slope" INTEGER NOT NULL,
    "area" FLOAT(53) NOT NULL,
    "job_id" UUID NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "blocks" ADD PRIMARY KEY("id");
CREATE INDEX "blocks_job_id_index" ON
    "blocks"("job_id");
CREATE INDEX "blocks_org_id_index" ON
    "blocks"("org_id");

CREATE TABLE "hours" (
    "id" UUID NOT NULL,
    "type" VARCHAR(255) CHECK
        ("type" IN('regular', 'overtime', 'travel', 'maintenance', 'lunch')) NOT NULL,
    "start_time" TIMESTAMP WITH TIME ZONE NOT NULL,
    "end_time" TIMESTAMP WITH TIME ZONE NOT NULL,
    "notes" VARCHAR(255) NOT NULL,
    "user_id" UUID NOT NULL,
    "job_id" UUID NOT NULL,
    "block_id" UUID NOT NULL,
    "phase_allocation_id" UUID NOT NULL,
    "machine_id" UUID NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "hours" ADD PRIMARY KEY("id");
CREATE INDEX "hours_user_id_index" ON
    "hours"("user_id");
CREATE INDEX "hours_job_id_index" ON
    "hours"("job_id");
CREATE INDEX "hours_block_id_index" ON
    "hours"("block_id");
CREATE INDEX "hours_phase_allocation_id_index" ON
    "hours"("phase_allocation_id");
CREATE INDEX "hours_machine_id_index" ON
    "hours"("machine_id");
CREATE INDEX "hours_org_id_index" ON
    "hours"("org_id");

CREATE TABLE "job_users" (
    "id" UUID NOT NULL,
    "role" VARCHAR(255) CHECK
        ("role" IN('manager', 'supervisor', 'operator', 'assistant')) NOT NULL,
    "job_id" UUID NOT NULL,
    "org_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "job_users" ADD PRIMARY KEY("id");
CREATE INDEX "job_users_job_id_index" ON
    "job_users"("job_id");
CREATE INDEX "job_users_org_id_index" ON
    "job_users"("org_id");
CREATE INDEX "job_users_user_id_index" ON
    "job_users"("user_id");

CREATE TABLE "block_tree_species" (
    "id" UUID NOT NULL,
    "volume" INTEGER NOT NULL,
    "tree_species_id" UUID NOT NULL,
    "block_id" UUID NOT NULL,
    "job_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "block_tree_species" ADD PRIMARY KEY("id");
CREATE INDEX "block_tree_species_tree_species_id_index" ON
    "block_tree_species"("tree_species_id");
CREATE INDEX "block_tree_species_block_id_index" ON
    "block_tree_species"("block_id");
CREATE INDEX "block_tree_species_job_id_index" ON
    "block_tree_species"("job_id");

CREATE TABLE "organizations" (
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "organizations" ADD PRIMARY KEY("id");

CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "first_name" VARCHAR(255) NOT NULL,
    "last_name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "phone_number" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    "is_supervisor" BOOLEAN NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "users" ADD PRIMARY KEY("id");
ALTER TABLE
    "users" ADD CONSTRAINT "users_email_unique" UNIQUE("email");
ALTER TABLE
    "users" ADD CONSTRAINT "users_phone_number_unique" UNIQUE("phone_number");
CREATE INDEX "users_org_id_index" ON
    "users"("org_id");

CREATE TABLE "tree_species" (
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "tree_species" ADD PRIMARY KEY("id");
ALTER TABLE
    "tree_species" ADD CONSTRAINT "tree_species_name_org_id_unique" UNIQUE ("name", "org_id");
CREATE INDEX "tree_species_org_id_index" ON
    "tree_species"("org_id");

CREATE TABLE "phases" (
    "id" UUID NOT NULL,
    "name" VARCHAR(255) CHECK
        ("name" IN('bunching', 'skidding', 'decking', 'processing', 'loading', 'brush_piling', 'supervision', 'safety', 'book_keeping', 'roads', 'pickups', 'padding_machines')) NOT NULL,
    "administration_deduction" INTEGER NOT NULL,
    "production_basis" INTEGER NOT NULL,
    "job_id" UUID NOT NULL,
    "block_id" UUID NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "phases" ADD PRIMARY KEY("id");
CREATE INDEX "phases_job_id_index" ON
    "phases"("job_id");
CREATE INDEX "phases_block_id_index" ON
    "phases"("block_id");
CREATE INDEX "phases_org_id_index" ON
    "phases"("org_id");

CREATE TABLE "jobs" (
    "id" UUID NOT NULL,
    "status" VARCHAR(255) CHECK
        ("status" IN('planned', 'in_progress', 'completed', 'on_hold', 'cancelled')) NOT NULL,
    "timbermark" VARCHAR(255) NOT NULL,
    "area" VARCHAR(255) NOT NULL,
    "customer_id" UUID NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "jobs" ADD PRIMARY KEY("id");
ALTER TABLE
    "jobs" ADD CONSTRAINT "jobs_timbermark_unique" UNIQUE("timbermark");
CREATE INDEX "jobs_customer_id_index" ON
    "jobs"("customer_id");
CREATE INDEX "jobs_org_id_index" ON
    "jobs"("org_id");

CREATE TABLE "phase_allocations" (
    "id" UUID NOT NULL,
    "status" VARCHAR(255) CHECK
        ("status" IN('scheduled', 'in_progress', 'completed', 'cancelled')) NOT NULL,
    "phase_id" UUID NOT NULL,
    "block_id" UUID NOT NULL,
    "job_id" UUID NOT NULL,
    "machine_id" UUID NOT NULL,
    "customer_id" UUID NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "phase_allocations" ADD PRIMARY KEY("id");
CREATE INDEX "phase_allocations_phase_id_index" ON
    "phase_allocations"("phase_id");
CREATE INDEX "phase_allocations_block_id_index" ON
    "phase_allocations"("block_id");
CREATE INDEX "phase_allocations_job_id_index" ON
    "phase_allocations"("job_id");
CREATE INDEX "phase_allocations_machine_id_index" ON
    "phase_allocations"("machine_id");
CREATE INDEX "phase_allocations_customer_id_index" ON
    "phase_allocations"("customer_id");
CREATE INDEX "phase_allocations_org_id_index" ON
    "phase_allocations"("org_id");

CREATE TABLE "machines" (
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "machines" ADD PRIMARY KEY("id");
ALTER TABLE
    "machines" ADD CONSTRAINT "machines_name_org_id_unique" UNIQUE ("name", "org_id");
CREATE INDEX "machines_org_id_index" ON
    "machines"("org_id");

CREATE TABLE "customers" (
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "customers" ADD PRIMARY KEY("id");
ALTER TABLE
    "customers" ADD CONSTRAINT "customers_name_org_id_unique" UNIQUE ("name", "org_id");
CREATE INDEX "customers_org_id_index" ON
    "customers"("org_id");

CREATE TABLE "customer_machine_rates" (
    "id" UUID NOT NULL,
    "hourly_rate" INTEGER NOT NULL,
    "machine_id" UUID NOT NULL,
    "customer_id" UUID NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);
ALTER TABLE
    "customer_machine_rates" ADD PRIMARY KEY("id");
ALTER TABLE
    "customer_machine_rates" ADD CONSTRAINT "customer_machine_rates_machine_id_customer_id_unique" UNIQUE ("machine_id", "customer_id");
CREATE INDEX "customer_machine_rates_machine_id_index" ON
    "customer_machine_rates"("machine_id");
CREATE INDEX "customer_machine_rates_customer_id_index" ON
    "customer_machine_rates"("customer_id");
CREATE INDEX "customer_machine_rates_org_id_index" ON
    "customer_machine_rates"("org_id");

ALTER TABLE
    "phases" ADD CONSTRAINT "phases_block_id_foreign" FOREIGN KEY("block_id") REFERENCES "blocks"("id");
ALTER TABLE
    "phase_allocations" ADD CONSTRAINT "phase_allocations_customer_id_foreign" FOREIGN KEY("customer_id") REFERENCES "customers"("id");
ALTER TABLE
    "phase_allocations" ADD CONSTRAINT "phase_allocations_job_id_foreign" FOREIGN KEY("job_id") REFERENCES "jobs"("id");
ALTER TABLE
    "blocks" ADD CONSTRAINT "blocks_job_id_foreign" FOREIGN KEY("job_id") REFERENCES "jobs"("id");
ALTER TABLE
    "hours" ADD CONSTRAINT "hours_user_id_foreign" FOREIGN KEY("user_id") REFERENCES "users"("id");
ALTER TABLE
    "job_users" ADD CONSTRAINT "job_users_user_id_foreign" FOREIGN KEY("user_id") REFERENCES "users"("id");
ALTER TABLE
    "hours" ADD CONSTRAINT "hours_machine_id_foreign" FOREIGN KEY("machine_id") REFERENCES "machines"("id");
ALTER TABLE
    "phases" ADD CONSTRAINT "phases_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "machines" ADD CONSTRAINT "machines_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "job_users" ADD CONSTRAINT "job_users_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "jobs" ADD CONSTRAINT "jobs_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "hours" ADD CONSTRAINT "hours_phase_allocation_id_foreign" FOREIGN KEY("phase_allocation_id") REFERENCES "phase_allocations"("id");
ALTER TABLE
    "phase_allocations" ADD CONSTRAINT "phase_allocations_machine_id_foreign" FOREIGN KEY("machine_id") REFERENCES "machines"("id");
ALTER TABLE
    "phase_allocations" ADD CONSTRAINT "phase_allocations_block_id_foreign" FOREIGN KEY("block_id") REFERENCES "blocks"("id");
ALTER TABLE
    "block_tree_species" ADD CONSTRAINT "block_tree_species_tree_species_id_foreign" FOREIGN KEY("tree_species_id") REFERENCES "tree_species"("id");
ALTER TABLE
    "phase_allocations" ADD CONSTRAINT "phase_allocations_phase_id_foreign" FOREIGN KEY("phase_id") REFERENCES "phases"("id");
ALTER TABLE
    "blocks" ADD CONSTRAINT "blocks_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "tree_species" ADD CONSTRAINT "tree_species_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "block_tree_species" ADD CONSTRAINT "block_tree_species_job_id_foreign" FOREIGN KEY("job_id") REFERENCES "jobs"("id");
ALTER TABLE
    "jobs" ADD CONSTRAINT "jobs_customer_id_foreign" FOREIGN KEY("customer_id") REFERENCES "customers"("id");
ALTER TABLE
    "users" ADD CONSTRAINT "users_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "hours" ADD CONSTRAINT "hours_job_id_foreign" FOREIGN KEY("job_id") REFERENCES "jobs"("id");
ALTER TABLE
    "hours" ADD CONSTRAINT "hours_block_id_foreign" FOREIGN KEY("block_id") REFERENCES "blocks"("id");
ALTER TABLE
    "customer_machine_rates" ADD CONSTRAINT "customer_machine_rates_customer_id_foreign" FOREIGN KEY("customer_id") REFERENCES "customers"("id");
ALTER TABLE
    "hours" ADD CONSTRAINT "hours_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "block_tree_species" ADD CONSTRAINT "block_tree_species_block_id_foreign" FOREIGN KEY("block_id") REFERENCES "blocks"("id");
ALTER TABLE
    "customers" ADD CONSTRAINT "customers_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "customer_machine_rates" ADD CONSTRAINT "customer_machine_rates_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "customer_machine_rates" ADD CONSTRAINT "customer_machine_rates_machine_id_foreign" FOREIGN KEY("machine_id") REFERENCES "machines"("id");
ALTER TABLE
    "phase_allocations" ADD CONSTRAINT "phase_allocations_org_id_foreign" FOREIGN KEY("org_id") REFERENCES "organizations"("id");
ALTER TABLE
    "phases" ADD CONSTRAINT "phases_job_id_foreign" FOREIGN KEY("job_id") REFERENCES "jobs"("id");

