#[doc = "Register `PRIC_IO288` reader"]
pub type R = crate::R<PricIo288Spec>;
#[doc = "Register `PRIC_IO288` writer"]
pub type W = crate::W<PricIo288Spec>;
#[doc = "Field `EnblWrGroup0OfIPC` reader - Enable Write Group #0 of IPC"]
pub type EnblWrGroup0ofIpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfIPC` writer - Enable Write Group #0 of IPC"]
pub type EnblWrGroup0ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfIPC` reader - Enable Write Group #1 of IPC"]
pub type EnblWrGroup1ofIpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfIPC` writer - Enable Write Group #1 of IPC"]
pub type EnblWrGroup1ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfIPC` reader - Enable Write Group #2 of IPC"]
pub type EnblWrGroup2ofIpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfIPC` writer - Enable Write Group #2 of IPC"]
pub type EnblWrGroup2ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfIPC` reader - Enable Write Group #3 of IPC"]
pub type EnblWrGroup3ofIpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfIPC` writer - Enable Write Group #3 of IPC"]
pub type EnblWrGroup3ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfIPC` reader - Enable Write Group #4 of IPC"]
pub type EnblWrGroup4ofIpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfIPC` writer - Enable Write Group #4 of IPC"]
pub type EnblWrGroup4ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfIPC` reader - Enable Write Group #5 of IPC"]
pub type EnblWrGroup5ofIpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfIPC` writer - Enable Write Group #5 of IPC"]
pub type EnblWrGroup5ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1288PRIC1_288\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1288pric12880500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1288pric12880500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1288pric12880500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12880500` reader - Enable Reset Tolerance of PRIC1288PRIC1_288\\[05:00\\]"]
pub type EnblRstToleranceOfPric1288pric12880500R =
    crate::BitReader<EnblRstToleranceOfPric1288pric12880500>;
impl EnblRstToleranceOfPric1288pric12880500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1288pric12880500 {
        match self.bits {
            false => EnblRstToleranceOfPric1288pric12880500::ResetBySrst,
            true => EnblRstToleranceOfPric1288pric12880500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12880500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12880500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12880500` writer - Enable Reset Tolerance of PRIC1288PRIC1_288\\[05:00\\]"]
pub type EnblRstToleranceOfPric1288pric12880500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1288pric12880500>;
impl<'a, REG> EnblRstToleranceOfPric1288pric12880500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12880500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12880500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12880600` reader - Enable Write Protection of PRIC1288PRIC1_288\\[06:00\\]"]
pub type EnblWrProtOfPric1288pric12880600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12880600` writer - Enable Write Protection of PRIC1288PRIC1_288\\[06:00\\]"]
pub type EnblWrProtOfPric1288pric12880600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfTIMER0` reader - Enable Write Group #0 of TIMER0"]
pub type EnblWrGroup0ofTimer0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER0` writer - Enable Write Group #0 of TIMER0"]
pub type EnblWrGroup0ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER0` reader - Enable Write Group #1 of TIMER0"]
pub type EnblWrGroup1ofTimer0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER0` writer - Enable Write Group #1 of TIMER0"]
pub type EnblWrGroup1ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER0` reader - Enable Write Group #2 of TIMER0"]
pub type EnblWrGroup2ofTimer0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER0` writer - Enable Write Group #2 of TIMER0"]
pub type EnblWrGroup2ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER0` reader - Enable Write Group #3 of TIMER0"]
pub type EnblWrGroup3ofTimer0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER0` writer - Enable Write Group #3 of TIMER0"]
pub type EnblWrGroup3ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER0` reader - Enable Write Group #4 of TIMER0"]
pub type EnblWrGroup4ofTimer0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER0` writer - Enable Write Group #4 of TIMER0"]
pub type EnblWrGroup4ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER0` reader - Enable Write Group #5 of TIMER0"]
pub type EnblWrGroup5ofTimer0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER0` writer - Enable Write Group #5 of TIMER0"]
pub type EnblWrGroup5ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1288PRIC1_288\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1288pric12881308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1288pric12881308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1288pric12881308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12881308` reader - Enable Reset Tolerance of PRIC1288PRIC1_288\\[13:08\\]"]
pub type EnblRstToleranceOfPric1288pric12881308R =
    crate::BitReader<EnblRstToleranceOfPric1288pric12881308>;
impl EnblRstToleranceOfPric1288pric12881308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1288pric12881308 {
        match self.bits {
            false => EnblRstToleranceOfPric1288pric12881308::ResetBySrst,
            true => EnblRstToleranceOfPric1288pric12881308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12881308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12881308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12881308` writer - Enable Reset Tolerance of PRIC1288PRIC1_288\\[13:08\\]"]
pub type EnblRstToleranceOfPric1288pric12881308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1288pric12881308>;
impl<'a, REG> EnblRstToleranceOfPric1288pric12881308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12881308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12881308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12881408` reader - Enable Write Protection of PRIC1288PRIC1_288\\[14:08\\]"]
pub type EnblWrProtOfPric1288pric12881408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12881408` writer - Enable Write Protection of PRIC1288PRIC1_288\\[14:08\\]"]
pub type EnblWrProtOfPric1288pric12881408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfTIMER1` reader - Enable Write Group #0 of TIMER1"]
pub type EnblWrGroup0ofTimer1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER1` writer - Enable Write Group #0 of TIMER1"]
pub type EnblWrGroup0ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER1` reader - Enable Write Group #1 of TIMER1"]
pub type EnblWrGroup1ofTimer1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER1` writer - Enable Write Group #1 of TIMER1"]
pub type EnblWrGroup1ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER1` reader - Enable Write Group #2 of TIMER1"]
pub type EnblWrGroup2ofTimer1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER1` writer - Enable Write Group #2 of TIMER1"]
pub type EnblWrGroup2ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER1` reader - Enable Write Group #3 of TIMER1"]
pub type EnblWrGroup3ofTimer1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER1` writer - Enable Write Group #3 of TIMER1"]
pub type EnblWrGroup3ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER1` reader - Enable Write Group #4 of TIMER1"]
pub type EnblWrGroup4ofTimer1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER1` writer - Enable Write Group #4 of TIMER1"]
pub type EnblWrGroup4ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER1` reader - Enable Write Group #5 of TIMER1"]
pub type EnblWrGroup5ofTimer1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER1` writer - Enable Write Group #5 of TIMER1"]
pub type EnblWrGroup5ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1288PRIC1_288\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1288pric12882116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1288pric12882116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1288pric12882116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12882116` reader - Enable Reset Tolerance of PRIC1288PRIC1_288\\[21:16\\]"]
pub type EnblRstToleranceOfPric1288pric12882116R =
    crate::BitReader<EnblRstToleranceOfPric1288pric12882116>;
impl EnblRstToleranceOfPric1288pric12882116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1288pric12882116 {
        match self.bits {
            false => EnblRstToleranceOfPric1288pric12882116::ResetBySrst,
            true => EnblRstToleranceOfPric1288pric12882116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12882116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12882116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12882116` writer - Enable Reset Tolerance of PRIC1288PRIC1_288\\[21:16\\]"]
pub type EnblRstToleranceOfPric1288pric12882116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1288pric12882116>;
impl<'a, REG> EnblRstToleranceOfPric1288pric12882116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12882116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12882116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12882216` reader - Enable Write Protection of PRIC1288PRIC1_288\\[22:16\\]"]
pub type EnblWrProtOfPric1288pric12882216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12882216` writer - Enable Write Protection of PRIC1288PRIC1_288\\[22:16\\]"]
pub type EnblWrProtOfPric1288pric12882216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfTIMER2` reader - Enable Write Group #0 of TIMER2"]
pub type EnblWrGroup0ofTimer2R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER2` writer - Enable Write Group #0 of TIMER2"]
pub type EnblWrGroup0ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER2` reader - Enable Write Group #1 of TIMER2"]
pub type EnblWrGroup1ofTimer2R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER2` writer - Enable Write Group #1 of TIMER2"]
pub type EnblWrGroup1ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER2` reader - Enable Write Group #2 of TIMER2"]
pub type EnblWrGroup2ofTimer2R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER2` writer - Enable Write Group #2 of TIMER2"]
pub type EnblWrGroup2ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER2` reader - Enable Write Group #3 of TIMER2"]
pub type EnblWrGroup3ofTimer2R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER2` writer - Enable Write Group #3 of TIMER2"]
pub type EnblWrGroup3ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER2` reader - Enable Write Group #4 of TIMER2"]
pub type EnblWrGroup4ofTimer2R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER2` writer - Enable Write Group #4 of TIMER2"]
pub type EnblWrGroup4ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER2` reader - Enable Write Group #5 of TIMER2"]
pub type EnblWrGroup5ofTimer2R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER2` writer - Enable Write Group #5 of TIMER2"]
pub type EnblWrGroup5ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1288PRIC1_288\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1288pric12882924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1288pric12882924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1288pric12882924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12882924` reader - Enable Reset Tolerance of PRIC1288PRIC1_288\\[29:24\\]"]
pub type EnblRstToleranceOfPric1288pric12882924R =
    crate::BitReader<EnblRstToleranceOfPric1288pric12882924>;
impl EnblRstToleranceOfPric1288pric12882924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1288pric12882924 {
        match self.bits {
            false => EnblRstToleranceOfPric1288pric12882924::ResetBySrst,
            true => EnblRstToleranceOfPric1288pric12882924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12882924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1288pric12882924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1288PRIC12882924` writer - Enable Reset Tolerance of PRIC1288PRIC1_288\\[29:24\\]"]
pub type EnblRstToleranceOfPric1288pric12882924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1288pric12882924>;
impl<'a, REG> EnblRstToleranceOfPric1288pric12882924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12882924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1288pric12882924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12883024` reader - Enable Write Protection of PRIC1288PRIC1_288\\[30:24\\]"]
pub type EnblWrProtOfPric1288pric12883024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1288PRIC12883024` writer - Enable Write Protection of PRIC1288PRIC1_288\\[30:24\\]"]
pub type EnblWrProtOfPric1288pric12883024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ipc(&self) -> EnblWrGroup0ofIpcR {
        EnblWrGroup0ofIpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ipc(&self) -> EnblWrGroup1ofIpcR {
        EnblWrGroup1ofIpcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ipc(&self) -> EnblWrGroup2ofIpcR {
        EnblWrGroup2ofIpcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ipc(&self) -> EnblWrGroup3ofIpcR {
        EnblWrGroup3ofIpcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ipc(&self) -> EnblWrGroup4ofIpcR {
        EnblWrGroup4ofIpcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ipc(&self) -> EnblWrGroup5ofIpcR {
        EnblWrGroup5ofIpcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12880500(
        &self,
    ) -> EnblRstToleranceOfPric1288pric12880500R {
        EnblRstToleranceOfPric1288pric12880500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1288PRIC1_288\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12880600(&self) -> EnblWrProtOfPric1288pric12880600R {
        EnblWrProtOfPric1288pric12880600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer0(&self) -> EnblWrGroup0ofTimer0R {
        EnblWrGroup0ofTimer0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer0(&self) -> EnblWrGroup1ofTimer0R {
        EnblWrGroup1ofTimer0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer0(&self) -> EnblWrGroup2ofTimer0R {
        EnblWrGroup2ofTimer0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer0(&self) -> EnblWrGroup3ofTimer0R {
        EnblWrGroup3ofTimer0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer0(&self) -> EnblWrGroup4ofTimer0R {
        EnblWrGroup4ofTimer0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer0(&self) -> EnblWrGroup5ofTimer0R {
        EnblWrGroup5ofTimer0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12881308(
        &self,
    ) -> EnblRstToleranceOfPric1288pric12881308R {
        EnblRstToleranceOfPric1288pric12881308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1288PRIC1_288\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12881408(&self) -> EnblWrProtOfPric1288pric12881408R {
        EnblWrProtOfPric1288pric12881408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer1(&self) -> EnblWrGroup0ofTimer1R {
        EnblWrGroup0ofTimer1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer1(&self) -> EnblWrGroup1ofTimer1R {
        EnblWrGroup1ofTimer1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer1(&self) -> EnblWrGroup2ofTimer1R {
        EnblWrGroup2ofTimer1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer1(&self) -> EnblWrGroup3ofTimer1R {
        EnblWrGroup3ofTimer1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer1(&self) -> EnblWrGroup4ofTimer1R {
        EnblWrGroup4ofTimer1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer1(&self) -> EnblWrGroup5ofTimer1R {
        EnblWrGroup5ofTimer1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12882116(
        &self,
    ) -> EnblRstToleranceOfPric1288pric12882116R {
        EnblRstToleranceOfPric1288pric12882116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1288PRIC1_288\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12882216(&self) -> EnblWrProtOfPric1288pric12882216R {
        EnblWrProtOfPric1288pric12882216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer2(&self) -> EnblWrGroup0ofTimer2R {
        EnblWrGroup0ofTimer2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer2(&self) -> EnblWrGroup1ofTimer2R {
        EnblWrGroup1ofTimer2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer2(&self) -> EnblWrGroup2ofTimer2R {
        EnblWrGroup2ofTimer2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer2(&self) -> EnblWrGroup3ofTimer2R {
        EnblWrGroup3ofTimer2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer2(&self) -> EnblWrGroup4ofTimer2R {
        EnblWrGroup4ofTimer2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer2(&self) -> EnblWrGroup5ofTimer2R {
        EnblWrGroup5ofTimer2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12882924(
        &self,
    ) -> EnblRstToleranceOfPric1288pric12882924R {
        EnblRstToleranceOfPric1288pric12882924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1288PRIC1_288\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12883024(&self) -> EnblWrProtOfPric1288pric12883024R {
        EnblWrProtOfPric1288pric12883024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ipc(&mut self) -> EnblWrGroup0ofIpcW<PricIo288Spec> {
        EnblWrGroup0ofIpcW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ipc(&mut self) -> EnblWrGroup1ofIpcW<PricIo288Spec> {
        EnblWrGroup1ofIpcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ipc(&mut self) -> EnblWrGroup2ofIpcW<PricIo288Spec> {
        EnblWrGroup2ofIpcW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ipc(&mut self) -> EnblWrGroup3ofIpcW<PricIo288Spec> {
        EnblWrGroup3ofIpcW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ipc(&mut self) -> EnblWrGroup4ofIpcW<PricIo288Spec> {
        EnblWrGroup4ofIpcW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of IPC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ipc(&mut self) -> EnblWrGroup5ofIpcW<PricIo288Spec> {
        EnblWrGroup5ofIpcW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12880500(
        &mut self,
    ) -> EnblRstToleranceOfPric1288pric12880500W<PricIo288Spec> {
        EnblRstToleranceOfPric1288pric12880500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1288PRIC1_288\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12880600(
        &mut self,
    ) -> EnblWrProtOfPric1288pric12880600W<PricIo288Spec> {
        EnblWrProtOfPric1288pric12880600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer0(&mut self) -> EnblWrGroup0ofTimer0W<PricIo288Spec> {
        EnblWrGroup0ofTimer0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer0(&mut self) -> EnblWrGroup1ofTimer0W<PricIo288Spec> {
        EnblWrGroup1ofTimer0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer0(&mut self) -> EnblWrGroup2ofTimer0W<PricIo288Spec> {
        EnblWrGroup2ofTimer0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer0(&mut self) -> EnblWrGroup3ofTimer0W<PricIo288Spec> {
        EnblWrGroup3ofTimer0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer0(&mut self) -> EnblWrGroup4ofTimer0W<PricIo288Spec> {
        EnblWrGroup4ofTimer0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of TIMER0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer0(&mut self) -> EnblWrGroup5ofTimer0W<PricIo288Spec> {
        EnblWrGroup5ofTimer0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12881308(
        &mut self,
    ) -> EnblRstToleranceOfPric1288pric12881308W<PricIo288Spec> {
        EnblRstToleranceOfPric1288pric12881308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1288PRIC1_288\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12881408(
        &mut self,
    ) -> EnblWrProtOfPric1288pric12881408W<PricIo288Spec> {
        EnblWrProtOfPric1288pric12881408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer1(&mut self) -> EnblWrGroup0ofTimer1W<PricIo288Spec> {
        EnblWrGroup0ofTimer1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer1(&mut self) -> EnblWrGroup1ofTimer1W<PricIo288Spec> {
        EnblWrGroup1ofTimer1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer1(&mut self) -> EnblWrGroup2ofTimer1W<PricIo288Spec> {
        EnblWrGroup2ofTimer1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer1(&mut self) -> EnblWrGroup3ofTimer1W<PricIo288Spec> {
        EnblWrGroup3ofTimer1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer1(&mut self) -> EnblWrGroup4ofTimer1W<PricIo288Spec> {
        EnblWrGroup4ofTimer1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of TIMER1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer1(&mut self) -> EnblWrGroup5ofTimer1W<PricIo288Spec> {
        EnblWrGroup5ofTimer1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12882116(
        &mut self,
    ) -> EnblRstToleranceOfPric1288pric12882116W<PricIo288Spec> {
        EnblRstToleranceOfPric1288pric12882116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1288PRIC1_288\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12882216(
        &mut self,
    ) -> EnblWrProtOfPric1288pric12882216W<PricIo288Spec> {
        EnblWrProtOfPric1288pric12882216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer2(&mut self) -> EnblWrGroup0ofTimer2W<PricIo288Spec> {
        EnblWrGroup0ofTimer2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer2(&mut self) -> EnblWrGroup1ofTimer2W<PricIo288Spec> {
        EnblWrGroup1ofTimer2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer2(&mut self) -> EnblWrGroup2ofTimer2W<PricIo288Spec> {
        EnblWrGroup2ofTimer2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer2(&mut self) -> EnblWrGroup3ofTimer2W<PricIo288Spec> {
        EnblWrGroup3ofTimer2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer2(&mut self) -> EnblWrGroup4ofTimer2W<PricIo288Spec> {
        EnblWrGroup4ofTimer2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of TIMER2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer2(&mut self) -> EnblWrGroup5ofTimer2W<PricIo288Spec> {
        EnblWrGroup5ofTimer2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1288PRIC1_288\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1288pric12882924(
        &mut self,
    ) -> EnblRstToleranceOfPric1288pric12882924W<PricIo288Spec> {
        EnblRstToleranceOfPric1288pric12882924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1288PRIC1_288\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1288pric12883024(
        &mut self,
    ) -> EnblWrProtOfPric1288pric12883024W<PricIo288Spec> {
        EnblWrProtOfPric1288pric12883024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io288::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io288::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo288Spec;
impl crate::RegisterSpec for PricIo288Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io288::R`](R) reader structure"]
impl crate::Readable for PricIo288Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io288::W`](W) writer structure"]
impl crate::Writable for PricIo288Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO288 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo288Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
