#[doc = "Register `SPIPF000` reader"]
pub type R = crate::R<Spipf000Spec>;
#[doc = "Register `SPIPF000` writer"]
pub type W = crate::W<Spipf000Spec>;
#[doc = "Field `EnblSingleBitPassthrough` reader - Enable Single Bit Passthrough"]
pub type EnblSingleBitPassthroughR = crate::BitReader;
#[doc = "Field `EnblSingleBitPassthrough` writer - Enable Single Bit Passthrough"]
pub type EnblSingleBitPassthroughW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMultipleBitPassthrough` reader - Enable Multiple Bit Passthrough"]
pub type EnblMultipleBitPassthroughR = crate::BitReader;
#[doc = "Field `EnblMultipleBitPassthrough` writer - Enable Multiple Bit Passthrough"]
pub type EnblMultipleBitPassthroughW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblFilterFn` reader - Enable Filter function"]
pub type EnblFilterFnR = crate::BitReader;
#[doc = "Field `EnblFilterFn` writer - Enable Filter function"]
pub type EnblFilterFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SPIMode` reader - SPI Mode"]
pub type SpimodeR = crate::FieldReader;
#[doc = "Field `SPIMode` writer - SPI Mode"]
pub type SpimodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Block Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockMode {
    #[doc = "1: Block a command by one extra CLK."]
    BlockACommandByOneExtraClk = 1,
    #[doc = "0: Block a command by deasserting CS early."]
    BlockACommandByDeassertingCsEarly = 0,
}
impl From<BlockMode> for bool {
    #[inline(always)]
    fn from(variant: BlockMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BlockMode` reader - Block Mode"]
pub type BlockModeR = crate::BitReader<BlockMode>;
impl BlockModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockMode {
        match self.bits {
            true => BlockMode::BlockACommandByOneExtraClk,
            false => BlockMode::BlockACommandByDeassertingCsEarly,
        }
    }
    #[doc = "Block a command by one extra CLK."]
    #[inline(always)]
    pub fn is_block_a_command_by_one_extra_clk(&self) -> bool {
        *self == BlockMode::BlockACommandByOneExtraClk
    }
    #[doc = "Block a command by deasserting CS early."]
    #[inline(always)]
    pub fn is_block_a_command_by_deasserting_cs_early(&self) -> bool {
        *self == BlockMode::BlockACommandByDeassertingCsEarly
    }
}
#[doc = "Field `BlockMode` writer - Block Mode"]
pub type BlockModeW<'a, REG> = crate::BitWriter<'a, REG, BlockMode>;
impl<'a, REG> BlockModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Block a command by one extra CLK."]
    #[inline(always)]
    pub fn block_a_command_by_one_extra_clk(self) -> &'a mut crate::W<REG> {
        self.variant(BlockMode::BlockACommandByOneExtraClk)
    }
    #[doc = "Block a command by deasserting CS early."]
    #[inline(always)]
    pub fn block_a_command_by_deasserting_cs_early(self) -> &'a mut crate::W<REG> {
        self.variant(BlockMode::BlockACommandByDeassertingCsEarly)
    }
}
#[doc = "Block FIFO Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockFifoclear {
    #[doc = "1: Clear Block FIFO"]
    ClearBlockFifo = 1,
}
impl From<BlockFifoclear> for bool {
    #[inline(always)]
    fn from(variant: BlockFifoclear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BlockFIFOClear` reader - Block FIFO Clear"]
pub type BlockFifoclearR = crate::BitReader<BlockFifoclear>;
impl BlockFifoclearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BlockFifoclear> {
        match self.bits {
            true => Some(BlockFifoclear::ClearBlockFifo),
            _ => None,
        }
    }
    #[doc = "Clear Block FIFO"]
    #[inline(always)]
    pub fn is_clear_block_fifo(&self) -> bool {
        *self == BlockFifoclear::ClearBlockFifo
    }
}
#[doc = "Field `BlockFIFOClear` writer - Block FIFO Clear"]
pub type BlockFifoclearW<'a, REG> = crate::BitWriter<'a, REG, BlockFifoclear>;
impl<'a, REG> BlockFifoclearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Block FIFO"]
    #[inline(always)]
    pub fn clear_block_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(BlockFifoclear::ClearBlockFifo)
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Drive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriveMode {
    #[doc = "1: Drive DataOut after CS is low."]
    DriveDataOutAfterCsIsLow = 1,
    #[doc = "0: Drive DataOut\\[0\\] (SPI) or DataOut\\[3:0\\] (QPI) before CS is low."]
    DriveDataOut0SpiOrDataOut30QpiBeforeCsIsLow = 0,
}
impl From<DriveMode> for bool {
    #[inline(always)]
    fn from(variant: DriveMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DriveMode` reader - Drive Mode"]
pub type DriveModeR = crate::BitReader<DriveMode>;
impl DriveModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DriveMode {
        match self.bits {
            true => DriveMode::DriveDataOutAfterCsIsLow,
            false => DriveMode::DriveDataOut0SpiOrDataOut30QpiBeforeCsIsLow,
        }
    }
    #[doc = "Drive DataOut after CS is low."]
    #[inline(always)]
    pub fn is_drive_data_out_after_cs_is_low(&self) -> bool {
        *self == DriveMode::DriveDataOutAfterCsIsLow
    }
    #[doc = "Drive DataOut\\[0\\] (SPI) or DataOut\\[3:0\\] (QPI) before CS is low."]
    #[inline(always)]
    pub fn is_drive_data_out0_spi_or_data_out30_qpi_before_cs_is_low(&self) -> bool {
        *self == DriveMode::DriveDataOut0SpiOrDataOut30QpiBeforeCsIsLow
    }
}
#[doc = "Field `DriveMode` writer - Drive Mode"]
pub type DriveModeW<'a, REG> = crate::BitWriter<'a, REG, DriveMode>;
impl<'a, REG> DriveModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drive DataOut after CS is low."]
    #[inline(always)]
    pub fn drive_data_out_after_cs_is_low(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode::DriveDataOutAfterCsIsLow)
    }
    #[doc = "Drive DataOut\\[0\\] (SPI) or DataOut\\[3:0\\] (QPI) before CS is low."]
    #[inline(always)]
    pub fn drive_data_out0_spi_or_data_out30_qpi_before_cs_is_low(self) -> &'a mut crate::W<REG> {
        self.variant(DriveMode::DriveDataOut0SpiOrDataOut30QpiBeforeCsIsLow)
    }
}
#[doc = "Block FIFO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockFifomode {
    #[doc = "1: Store the latest. The lastest orverrides."]
    StoreTheLatestTheLastestOrverrides = 1,
    #[doc = "0: Store the oldest. Stops when FIFO full."]
    StoreTheOldestStopsWhenFifoFull = 0,
}
impl From<BlockFifomode> for bool {
    #[inline(always)]
    fn from(variant: BlockFifomode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BlockFIFOMode` reader - Block FIFO Mode"]
pub type BlockFifomodeR = crate::BitReader<BlockFifomode>;
impl BlockFifomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockFifomode {
        match self.bits {
            true => BlockFifomode::StoreTheLatestTheLastestOrverrides,
            false => BlockFifomode::StoreTheOldestStopsWhenFifoFull,
        }
    }
    #[doc = "Store the latest. The lastest orverrides."]
    #[inline(always)]
    pub fn is_store_the_latest_the_lastest_orverrides(&self) -> bool {
        *self == BlockFifomode::StoreTheLatestTheLastestOrverrides
    }
    #[doc = "Store the oldest. Stops when FIFO full."]
    #[inline(always)]
    pub fn is_store_the_oldest_stops_when_fifo_full(&self) -> bool {
        *self == BlockFifomode::StoreTheOldestStopsWhenFifoFull
    }
}
#[doc = "Field `BlockFIFOMode` writer - Block FIFO Mode"]
pub type BlockFifomodeW<'a, REG> = crate::BitWriter<'a, REG, BlockFifomode>;
impl<'a, REG> BlockFifomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Store the latest. The lastest orverrides."]
    #[inline(always)]
    pub fn store_the_latest_the_lastest_orverrides(self) -> &'a mut crate::W<REG> {
        self.variant(BlockFifomode::StoreTheLatestTheLastestOrverrides)
    }
    #[doc = "Store the oldest. Stops when FIFO full."]
    #[inline(always)]
    pub fn store_the_oldest_stops_when_fifo_full(self) -> &'a mut crate::W<REG> {
        self.variant(BlockFifomode::StoreTheOldestStopsWhenFifoFull)
    }
}
#[doc = "SW Engine Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwengRst {
    #[doc = "1: Reset"]
    Reset = 1,
    #[doc = "0: NOP"]
    Nop = 0,
}
impl From<SwengRst> for bool {
    #[inline(always)]
    fn from(variant: SwengRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWEngRst` reader - SW Engine Reset"]
pub type SwengRstR = crate::BitReader<SwengRst>;
impl SwengRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwengRst {
        match self.bits {
            true => SwengRst::Reset,
            false => SwengRst::Nop,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SwengRst::Reset
    }
    #[doc = "NOP"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SwengRst::Nop
    }
}
#[doc = "Field `SWEngRst` writer - SW Engine Reset"]
pub type SwengRstW<'a, REG> = crate::BitWriter<'a, REG, SwengRst>;
impl<'a, REG> SwengRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SwengRst::Reset)
    }
    #[doc = "NOP"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(SwengRst::Nop)
    }
}
#[doc = "Field `EnblDebounceOfCKIn` reader - Enable Debounce of CKIn"]
pub type EnblDebounceOfCkinR = crate::BitReader;
#[doc = "Field `EnblDebounceOfCKIn` writer - Enable Debounce of CKIn"]
pub type EnblDebounceOfCkinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDebounceOfCSIn` reader - Enable Debounce of CSIn"]
pub type EnblDebounceOfCsinR = crate::BitReader;
#[doc = "Field `EnblDebounceOfCSIn` writer - Enable Debounce of CSIn"]
pub type EnblDebounceOfCsinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDebounceOfDataIn0` reader - Enable Debounce of DataIn\\[0\\]"]
pub type EnblDebounceOfDataIn0R = crate::BitReader;
#[doc = "Field `EnblDebounceOfDataIn0` writer - Enable Debounce of DataIn\\[0\\]"]
pub type EnblDebounceOfDataIn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDebounceOfDataIn1` reader - Enable Debounce of DataIn\\[1\\]"]
pub type EnblDebounceOfDataIn1R = crate::BitReader;
#[doc = "Field `EnblDebounceOfDataIn1` writer - Enable Debounce of DataIn\\[1\\]"]
pub type EnblDebounceOfDataIn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDebounceOfDataIn2` reader - Enable Debounce of DataIn\\[2\\]"]
pub type EnblDebounceOfDataIn2R = crate::BitReader;
#[doc = "Field `EnblDebounceOfDataIn2` writer - Enable Debounce of DataIn\\[2\\]"]
pub type EnblDebounceOfDataIn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDebounceOfDataIn3` reader - Enable Debounce of DataIn\\[3\\]"]
pub type EnblDebounceOfDataIn3R = crate::BitReader;
#[doc = "Field `EnblDebounceOfDataIn3` writer - Enable Debounce of DataIn\\[3\\]"]
pub type EnblDebounceOfDataIn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF0008` reader - Write Disable of SPIPF000\\[8\\]"]
pub type WrDisOfSpipf0008R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF0008` writer - Write Disable of SPIPF000\\[8\\]"]
pub type WrDisOfSpipf0008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF00015` reader - Write Disable of SPIPF000\\[15\\]"]
pub type WrDisOfSpipf00015R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF00015` writer - Write Disable of SPIPF000\\[15\\]"]
pub type WrDisOfSpipf00015W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlockFIFOLen` reader - Block FIFO length"]
pub type BlockFifolenR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable Single Bit Passthrough"]
    #[inline(always)]
    pub fn enbl_single_bit_passthrough(&self) -> EnblSingleBitPassthroughR {
        EnblSingleBitPassthroughR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Multiple Bit Passthrough"]
    #[inline(always)]
    pub fn enbl_multiple_bit_passthrough(&self) -> EnblMultipleBitPassthroughR {
        EnblMultipleBitPassthroughR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Filter function"]
    #[inline(always)]
    pub fn enbl_filter_fn(&self) -> EnblFilterFnR {
        EnblFilterFnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Block Mode"]
    #[inline(always)]
    pub fn block_mode(&self) -> BlockModeR {
        BlockModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Block FIFO Clear"]
    #[inline(always)]
    pub fn block_fifoclear(&self) -> BlockFifoclearR {
        BlockFifoclearR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Drive Mode"]
    #[inline(always)]
    pub fn drive_mode(&self) -> DriveModeR {
        DriveModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Block FIFO Mode"]
    #[inline(always)]
    pub fn block_fifomode(&self) -> BlockFifomodeR {
        BlockFifomodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - SW Engine Reset"]
    #[inline(always)]
    pub fn sweng_rst(&self) -> SwengRstR {
        SwengRstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Debounce of CKIn"]
    #[inline(always)]
    pub fn enbl_debounce_of_ckin(&self) -> EnblDebounceOfCkinR {
        EnblDebounceOfCkinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Debounce of CSIn"]
    #[inline(always)]
    pub fn enbl_debounce_of_csin(&self) -> EnblDebounceOfCsinR {
        EnblDebounceOfCsinR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Debounce of DataIn\\[0\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in0(&self) -> EnblDebounceOfDataIn0R {
        EnblDebounceOfDataIn0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Debounce of DataIn\\[1\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in1(&self) -> EnblDebounceOfDataIn1R {
        EnblDebounceOfDataIn1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Debounce of DataIn\\[2\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in2(&self) -> EnblDebounceOfDataIn2R {
        EnblDebounceOfDataIn2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Debounce of DataIn\\[3\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in3(&self) -> EnblDebounceOfDataIn3R {
        EnblDebounceOfDataIn3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write Disable of SPIPF000\\[8\\]"]
    #[inline(always)]
    pub fn wr_dis_of_spipf0008(&self) -> WrDisOfSpipf0008R {
        WrDisOfSpipf0008R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write Disable of SPIPF000\\[15\\]"]
    #[inline(always)]
    pub fn wr_dis_of_spipf00015(&self) -> WrDisOfSpipf00015R {
        WrDisOfSpipf00015R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Block FIFO length"]
    #[inline(always)]
    pub fn block_fifolen(&self) -> BlockFifolenR {
        BlockFifolenR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Single Bit Passthrough"]
    #[inline(always)]
    pub fn enbl_single_bit_passthrough(&mut self) -> EnblSingleBitPassthroughW<Spipf000Spec> {
        EnblSingleBitPassthroughW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Multiple Bit Passthrough"]
    #[inline(always)]
    pub fn enbl_multiple_bit_passthrough(&mut self) -> EnblMultipleBitPassthroughW<Spipf000Spec> {
        EnblMultipleBitPassthroughW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Filter function"]
    #[inline(always)]
    pub fn enbl_filter_fn(&mut self) -> EnblFilterFnW<Spipf000Spec> {
        EnblFilterFnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline(always)]
    pub fn spimode(&mut self) -> SpimodeW<Spipf000Spec> {
        SpimodeW::new(self, 4)
    }
    #[doc = "Bit 7 - Block Mode"]
    #[inline(always)]
    pub fn block_mode(&mut self) -> BlockModeW<Spipf000Spec> {
        BlockModeW::new(self, 7)
    }
    #[doc = "Bit 8 - Block FIFO Clear"]
    #[inline(always)]
    pub fn block_fifoclear(&mut self) -> BlockFifoclearW<Spipf000Spec> {
        BlockFifoclearW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Spipf000Spec> {
        Reserved1W::new(self, 9)
    }
    #[doc = "Bit 10 - Drive Mode"]
    #[inline(always)]
    pub fn drive_mode(&mut self) -> DriveModeW<Spipf000Spec> {
        DriveModeW::new(self, 10)
    }
    #[doc = "Bit 11 - Block FIFO Mode"]
    #[inline(always)]
    pub fn block_fifomode(&mut self) -> BlockFifomodeW<Spipf000Spec> {
        BlockFifomodeW::new(self, 11)
    }
    #[doc = "Bit 15 - SW Engine Reset"]
    #[inline(always)]
    pub fn sweng_rst(&mut self) -> SwengRstW<Spipf000Spec> {
        SwengRstW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Debounce of CKIn"]
    #[inline(always)]
    pub fn enbl_debounce_of_ckin(&mut self) -> EnblDebounceOfCkinW<Spipf000Spec> {
        EnblDebounceOfCkinW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Debounce of CSIn"]
    #[inline(always)]
    pub fn enbl_debounce_of_csin(&mut self) -> EnblDebounceOfCsinW<Spipf000Spec> {
        EnblDebounceOfCsinW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Debounce of DataIn\\[0\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in0(&mut self) -> EnblDebounceOfDataIn0W<Spipf000Spec> {
        EnblDebounceOfDataIn0W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Debounce of DataIn\\[1\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in1(&mut self) -> EnblDebounceOfDataIn1W<Spipf000Spec> {
        EnblDebounceOfDataIn1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Debounce of DataIn\\[2\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in2(&mut self) -> EnblDebounceOfDataIn2W<Spipf000Spec> {
        EnblDebounceOfDataIn2W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Debounce of DataIn\\[3\\]"]
    #[inline(always)]
    pub fn enbl_debounce_of_data_in3(&mut self) -> EnblDebounceOfDataIn3W<Spipf000Spec> {
        EnblDebounceOfDataIn3W::new(self, 21)
    }
    #[doc = "Bit 22 - Write Disable of SPIPF000\\[8\\]"]
    #[inline(always)]
    pub fn wr_dis_of_spipf0008(&mut self) -> WrDisOfSpipf0008W<Spipf000Spec> {
        WrDisOfSpipf0008W::new(self, 22)
    }
    #[doc = "Bit 23 - Write Disable of SPIPF000\\[15\\]"]
    #[inline(always)]
    pub fn wr_dis_of_spipf00015(&mut self) -> WrDisOfSpipf00015W<Spipf000Spec> {
        WrDisOfSpipf00015W::new(self, 23)
    }
}
#[doc = "Engine Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf000Spec;
impl crate::RegisterSpec for Spipf000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf000::R`](R) reader structure"]
impl crate::Readable for Spipf000Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf000::W`](W) writer structure"]
impl crate::Writable for Spipf000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF000 to value 0"]
impl crate::Resettable for Spipf000Spec {}
