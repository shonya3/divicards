/*
pub fn write_sold(initial_path: &str, after_path: &str) -> Result<(), Error> {
    let initial_vec = get_records_from_csv(initial_path, None)?;
    let after_vec = get_records_from_csv(after_path, None)?;
    write_to_csv(&initial_vec, "output_initial.csv")?;

    let mut initial_map = HashMap::<String, Record>::new();
    let mut after_map = HashMap::<String, Record>::new();

    for record in initial_vec {
        initial_map.insert(record.name.clone(), record);
    }

    for record in after_vec {
        after_map.insert(record.name.clone(), record);
    }

    let mut sold_vec = Vec::<Record>::new();
    for (name, record) in initial_map {
        let after_record = after_map.entry(name.clone()).or_insert(Record::default());
        after_record.name = record.name.clone();
        after_record.calculated = record.calculated;

        let calculated = match record.calculated {
            Some(price) => price,
            None => match &name {
                _ if record::is_legacy_card(&name) => 0.0,
                _ => return Err(Error::NoPriceError(name.to_string())),
            },
        };

        let stack_size = record.stack_size - after_record.stack_size;
        let total = calculated * stack_size as f32;

        let sold_record = Record {
            name: name.clone(),
            calculated: Some(calculated),
            stack_size,
            total: Some(total),
        };

        sold_vec.push(sold_record);
    }

    let nullified_after_vector = after_map.values().cloned().collect::<Vec<Record>>();

    write_to_csv(&nullified_after_vector, "output_after.csv")?;
    write_to_csv(&sold_vec, "output_sold.csv")?;

    Ok(())
}
*/

/*
pub fn write_add_sold_to_final(final_path: &str, sold_path: &str) -> Result<(), Error> {
    let final_vec = get_records_from_csv(final_path, None)?;
    let sold_vec = get_records_from_csv(sold_path, None)?;

    let mut final_map = HashMap::<String, Record>::new();
    for record in final_vec {
        final_map.insert(record.name.clone(), record);
    }

    let mut sold_map = HashMap::<String, Record>::new();
    for record in sold_vec {
        sold_map.insert(record.name.clone(), record);
    }

    let mut summarized_vec: Vec<Record> = vec![];
    for (name, record) in sold_map {
        let mut final_record = final_map.entry(name.clone()).or_insert(Record::default());
        final_record.name = name.clone();
        final_record.calculated = record.calculated;

        let calculated = record.calculated;
        let stack_size = record.stack_size + final_record.stack_size;
        let total = calculated * stack_size as f32;
        let summarized_record = Record {
            stack_size,
            name,
            calculated,
            total,
        };

        summarized_vec.push(summarized_record);
    }

    write_to_csv(&summarized_vec, "output_final_plus_sold.csv")?;

    Ok(())
}*/

/*
pub fn add_sold_diff_in_one_function(
    initial_path: &str,
    after_path: &str,
    final_path: &str,
) -> Result<(), Error> {
    write_sold(initial_path, after_path)?;
    write_add_sold_to_final(final_path, "output_sold.csv")?;

    Ok(())
}*/
