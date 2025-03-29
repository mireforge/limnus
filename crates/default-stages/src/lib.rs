/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use limnus_stage::StageTag;

pub struct First;
impl StageTag for First {}

pub struct PreUpdate;
impl StageTag for PreUpdate {}

pub struct Update;
impl StageTag for Update {}

pub struct PostUpdate;
impl StageTag for PostUpdate {}

pub struct FixedFirst;
impl StageTag for FixedFirst {}

pub struct FixedPreUpdate;
impl StageTag for FixedPreUpdate {}

pub struct FixedUpdate;
impl StageTag for FixedUpdate {}

pub struct FixedPostUpdate;
impl StageTag for FixedPostUpdate {}

pub struct RenderFirst;
impl StageTag for RenderFirst {}

pub struct RenderPreUpdate;
impl StageTag for RenderPreUpdate {}

pub struct RenderUpdate;
impl StageTag for RenderUpdate {}

pub struct RenderPostUpdate;
impl StageTag for RenderPostUpdate {}
