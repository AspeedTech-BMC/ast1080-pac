#[doc = "Register `PRIC_IO388` reader"]
pub type R = crate::R<PricIo388Spec>;
#[doc = "Register `PRIC_IO388` writer"]
pub type W = crate::W<PricIo388Spec>;
#[doc = "Field `EnblReadGroup0OfIPC` reader - Enable Read Group #0 of IPC"]
pub type EnblReadGroup0ofIpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfIPC` writer - Enable Read Group #0 of IPC"]
pub type EnblReadGroup0ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfIPC` reader - Enable Read Group #1 of IPC"]
pub type EnblReadGroup1ofIpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfIPC` writer - Enable Read Group #1 of IPC"]
pub type EnblReadGroup1ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfIPC` reader - Enable Read Group #2 of IPC"]
pub type EnblReadGroup2ofIpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfIPC` writer - Enable Read Group #2 of IPC"]
pub type EnblReadGroup2ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfIPC` reader - Enable Read Group #3 of IPC"]
pub type EnblReadGroup3ofIpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfIPC` writer - Enable Read Group #3 of IPC"]
pub type EnblReadGroup3ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfIPC` reader - Enable Read Group #4 of IPC"]
pub type EnblReadGroup4ofIpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfIPC` writer - Enable Read Group #4 of IPC"]
pub type EnblReadGroup4ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfIPC` reader - Enable Read Group #5 of IPC"]
pub type EnblReadGroup5ofIpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfIPC` writer - Enable Read Group #5 of IPC"]
pub type EnblReadGroup5ofIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1388PRIC1_388\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1388pric13880500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1388pric13880500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1388pric13880500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13880500` reader - Enable Reset Tolerance of PRIC1388PRIC1_388\\[05:00\\]"]
pub type EnblRstToleranceOfPric1388pric13880500R =
    crate::BitReader<EnblRstToleranceOfPric1388pric13880500>;
impl EnblRstToleranceOfPric1388pric13880500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1388pric13880500 {
        match self.bits {
            false => EnblRstToleranceOfPric1388pric13880500::ResetBySrst,
            true => EnblRstToleranceOfPric1388pric13880500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13880500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13880500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13880500` writer - Enable Reset Tolerance of PRIC1388PRIC1_388\\[05:00\\]"]
pub type EnblRstToleranceOfPric1388pric13880500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1388pric13880500>;
impl<'a, REG> EnblRstToleranceOfPric1388pric13880500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13880500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13880500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13880600` reader - Enable Write Protection of PRIC1388PRIC1_388\\[06:00\\]"]
pub type EnblWrProtOfPric1388pric13880600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13880600` writer - Enable Write Protection of PRIC1388PRIC1_388\\[06:00\\]"]
pub type EnblWrProtOfPric1388pric13880600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfTIMER0` reader - Enable Read Group #0 of TIMER0"]
pub type EnblReadGroup0ofTimer0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER0` writer - Enable Read Group #0 of TIMER0"]
pub type EnblReadGroup0ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER0` reader - Enable Read Group #1 of TIMER0"]
pub type EnblReadGroup1ofTimer0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER0` writer - Enable Read Group #1 of TIMER0"]
pub type EnblReadGroup1ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER0` reader - Enable Read Group #2 of TIMER0"]
pub type EnblReadGroup2ofTimer0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER0` writer - Enable Read Group #2 of TIMER0"]
pub type EnblReadGroup2ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER0` reader - Enable Read Group #3 of TIMER0"]
pub type EnblReadGroup3ofTimer0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER0` writer - Enable Read Group #3 of TIMER0"]
pub type EnblReadGroup3ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER0` reader - Enable Read Group #4 of TIMER0"]
pub type EnblReadGroup4ofTimer0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER0` writer - Enable Read Group #4 of TIMER0"]
pub type EnblReadGroup4ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER0` reader - Enable Read Group #5 of TIMER0"]
pub type EnblReadGroup5ofTimer0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER0` writer - Enable Read Group #5 of TIMER0"]
pub type EnblReadGroup5ofTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1388PRIC1_388\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1388pric13881308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1388pric13881308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1388pric13881308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13881308` reader - Enable Reset Tolerance of PRIC1388PRIC1_388\\[13:08\\]"]
pub type EnblRstToleranceOfPric1388pric13881308R =
    crate::BitReader<EnblRstToleranceOfPric1388pric13881308>;
impl EnblRstToleranceOfPric1388pric13881308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1388pric13881308 {
        match self.bits {
            false => EnblRstToleranceOfPric1388pric13881308::ResetBySrst,
            true => EnblRstToleranceOfPric1388pric13881308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13881308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13881308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13881308` writer - Enable Reset Tolerance of PRIC1388PRIC1_388\\[13:08\\]"]
pub type EnblRstToleranceOfPric1388pric13881308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1388pric13881308>;
impl<'a, REG> EnblRstToleranceOfPric1388pric13881308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13881308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13881308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13881408` reader - Enable Write Protection of PRIC1388PRIC1_388\\[14:08\\]"]
pub type EnblWrProtOfPric1388pric13881408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13881408` writer - Enable Write Protection of PRIC1388PRIC1_388\\[14:08\\]"]
pub type EnblWrProtOfPric1388pric13881408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfTIMER1` reader - Enable Read Group #0 of TIMER1"]
pub type EnblReadGroup0ofTimer1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER1` writer - Enable Read Group #0 of TIMER1"]
pub type EnblReadGroup0ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER1` reader - Enable Read Group #1 of TIMER1"]
pub type EnblReadGroup1ofTimer1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER1` writer - Enable Read Group #1 of TIMER1"]
pub type EnblReadGroup1ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER1` reader - Enable Read Group #2 of TIMER1"]
pub type EnblReadGroup2ofTimer1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER1` writer - Enable Read Group #2 of TIMER1"]
pub type EnblReadGroup2ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER1` reader - Enable Read Group #3 of TIMER1"]
pub type EnblReadGroup3ofTimer1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER1` writer - Enable Read Group #3 of TIMER1"]
pub type EnblReadGroup3ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER1` reader - Enable Read Group #4 of TIMER1"]
pub type EnblReadGroup4ofTimer1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER1` writer - Enable Read Group #4 of TIMER1"]
pub type EnblReadGroup4ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER1` reader - Enable Read Group #5 of TIMER1"]
pub type EnblReadGroup5ofTimer1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER1` writer - Enable Read Group #5 of TIMER1"]
pub type EnblReadGroup5ofTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1388PRIC1_388\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1388pric13882116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1388pric13882116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1388pric13882116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13882116` reader - Enable Reset Tolerance of PRIC1388PRIC1_388\\[21:16\\]"]
pub type EnblRstToleranceOfPric1388pric13882116R =
    crate::BitReader<EnblRstToleranceOfPric1388pric13882116>;
impl EnblRstToleranceOfPric1388pric13882116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1388pric13882116 {
        match self.bits {
            false => EnblRstToleranceOfPric1388pric13882116::ResetBySrst,
            true => EnblRstToleranceOfPric1388pric13882116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13882116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13882116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13882116` writer - Enable Reset Tolerance of PRIC1388PRIC1_388\\[21:16\\]"]
pub type EnblRstToleranceOfPric1388pric13882116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1388pric13882116>;
impl<'a, REG> EnblRstToleranceOfPric1388pric13882116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13882116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13882116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13882216` reader - Enable Write Protection of PRIC1388PRIC1_388\\[22:16\\]"]
pub type EnblWrProtOfPric1388pric13882216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13882216` writer - Enable Write Protection of PRIC1388PRIC1_388\\[22:16\\]"]
pub type EnblWrProtOfPric1388pric13882216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfTIMER2` reader - Enable Read Group #0 of TIMER2"]
pub type EnblReadGroup0ofTimer2R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER2` writer - Enable Read Group #0 of TIMER2"]
pub type EnblReadGroup0ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER2` reader - Enable Read Group #1 of TIMER2"]
pub type EnblReadGroup1ofTimer2R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER2` writer - Enable Read Group #1 of TIMER2"]
pub type EnblReadGroup1ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER2` reader - Enable Read Group #2 of TIMER2"]
pub type EnblReadGroup2ofTimer2R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER2` writer - Enable Read Group #2 of TIMER2"]
pub type EnblReadGroup2ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER2` reader - Enable Read Group #3 of TIMER2"]
pub type EnblReadGroup3ofTimer2R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER2` writer - Enable Read Group #3 of TIMER2"]
pub type EnblReadGroup3ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER2` reader - Enable Read Group #4 of TIMER2"]
pub type EnblReadGroup4ofTimer2R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER2` writer - Enable Read Group #4 of TIMER2"]
pub type EnblReadGroup4ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER2` reader - Enable Read Group #5 of TIMER2"]
pub type EnblReadGroup5ofTimer2R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER2` writer - Enable Read Group #5 of TIMER2"]
pub type EnblReadGroup5ofTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1388PRIC1_388\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1388pric13882924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1388pric13882924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1388pric13882924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13882924` reader - Enable Reset Tolerance of PRIC1388PRIC1_388\\[29:24\\]"]
pub type EnblRstToleranceOfPric1388pric13882924R =
    crate::BitReader<EnblRstToleranceOfPric1388pric13882924>;
impl EnblRstToleranceOfPric1388pric13882924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1388pric13882924 {
        match self.bits {
            false => EnblRstToleranceOfPric1388pric13882924::ResetBySrst,
            true => EnblRstToleranceOfPric1388pric13882924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13882924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1388pric13882924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1388PRIC13882924` writer - Enable Reset Tolerance of PRIC1388PRIC1_388\\[29:24\\]"]
pub type EnblRstToleranceOfPric1388pric13882924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1388pric13882924>;
impl<'a, REG> EnblRstToleranceOfPric1388pric13882924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13882924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1388pric13882924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13883024` reader - Enable Write Protection of PRIC1388PRIC1_388\\[30:24\\]"]
pub type EnblWrProtOfPric1388pric13883024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1388PRIC13883024` writer - Enable Write Protection of PRIC1388PRIC1_388\\[30:24\\]"]
pub type EnblWrProtOfPric1388pric13883024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group0of_ipc(&self) -> EnblReadGroup0ofIpcR {
        EnblReadGroup0ofIpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group1of_ipc(&self) -> EnblReadGroup1ofIpcR {
        EnblReadGroup1ofIpcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group2of_ipc(&self) -> EnblReadGroup2ofIpcR {
        EnblReadGroup2ofIpcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group3of_ipc(&self) -> EnblReadGroup3ofIpcR {
        EnblReadGroup3ofIpcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group4of_ipc(&self) -> EnblReadGroup4ofIpcR {
        EnblReadGroup4ofIpcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group5of_ipc(&self) -> EnblReadGroup5ofIpcR {
        EnblReadGroup5ofIpcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13880500(
        &self,
    ) -> EnblRstToleranceOfPric1388pric13880500R {
        EnblRstToleranceOfPric1388pric13880500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1388PRIC1_388\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13880600(&self) -> EnblWrProtOfPric1388pric13880600R {
        EnblWrProtOfPric1388pric13880600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer0(&self) -> EnblReadGroup0ofTimer0R {
        EnblReadGroup0ofTimer0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer0(&self) -> EnblReadGroup1ofTimer0R {
        EnblReadGroup1ofTimer0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer0(&self) -> EnblReadGroup2ofTimer0R {
        EnblReadGroup2ofTimer0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer0(&self) -> EnblReadGroup3ofTimer0R {
        EnblReadGroup3ofTimer0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer0(&self) -> EnblReadGroup4ofTimer0R {
        EnblReadGroup4ofTimer0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer0(&self) -> EnblReadGroup5ofTimer0R {
        EnblReadGroup5ofTimer0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13881308(
        &self,
    ) -> EnblRstToleranceOfPric1388pric13881308R {
        EnblRstToleranceOfPric1388pric13881308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1388PRIC1_388\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13881408(&self) -> EnblWrProtOfPric1388pric13881408R {
        EnblWrProtOfPric1388pric13881408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer1(&self) -> EnblReadGroup0ofTimer1R {
        EnblReadGroup0ofTimer1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer1(&self) -> EnblReadGroup1ofTimer1R {
        EnblReadGroup1ofTimer1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer1(&self) -> EnblReadGroup2ofTimer1R {
        EnblReadGroup2ofTimer1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer1(&self) -> EnblReadGroup3ofTimer1R {
        EnblReadGroup3ofTimer1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer1(&self) -> EnblReadGroup4ofTimer1R {
        EnblReadGroup4ofTimer1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer1(&self) -> EnblReadGroup5ofTimer1R {
        EnblReadGroup5ofTimer1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13882116(
        &self,
    ) -> EnblRstToleranceOfPric1388pric13882116R {
        EnblRstToleranceOfPric1388pric13882116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1388PRIC1_388\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13882216(&self) -> EnblWrProtOfPric1388pric13882216R {
        EnblWrProtOfPric1388pric13882216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer2(&self) -> EnblReadGroup0ofTimer2R {
        EnblReadGroup0ofTimer2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer2(&self) -> EnblReadGroup1ofTimer2R {
        EnblReadGroup1ofTimer2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer2(&self) -> EnblReadGroup2ofTimer2R {
        EnblReadGroup2ofTimer2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer2(&self) -> EnblReadGroup3ofTimer2R {
        EnblReadGroup3ofTimer2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer2(&self) -> EnblReadGroup4ofTimer2R {
        EnblReadGroup4ofTimer2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer2(&self) -> EnblReadGroup5ofTimer2R {
        EnblReadGroup5ofTimer2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13882924(
        &self,
    ) -> EnblRstToleranceOfPric1388pric13882924R {
        EnblRstToleranceOfPric1388pric13882924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1388PRIC1_388\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13883024(&self) -> EnblWrProtOfPric1388pric13883024R {
        EnblWrProtOfPric1388pric13883024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group0of_ipc(&mut self) -> EnblReadGroup0ofIpcW<PricIo388Spec> {
        EnblReadGroup0ofIpcW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group1of_ipc(&mut self) -> EnblReadGroup1ofIpcW<PricIo388Spec> {
        EnblReadGroup1ofIpcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group2of_ipc(&mut self) -> EnblReadGroup2ofIpcW<PricIo388Spec> {
        EnblReadGroup2ofIpcW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group3of_ipc(&mut self) -> EnblReadGroup3ofIpcW<PricIo388Spec> {
        EnblReadGroup3ofIpcW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group4of_ipc(&mut self) -> EnblReadGroup4ofIpcW<PricIo388Spec> {
        EnblReadGroup4ofIpcW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of IPC"]
    #[inline(always)]
    pub fn enbl_read_group5of_ipc(&mut self) -> EnblReadGroup5ofIpcW<PricIo388Spec> {
        EnblReadGroup5ofIpcW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13880500(
        &mut self,
    ) -> EnblRstToleranceOfPric1388pric13880500W<PricIo388Spec> {
        EnblRstToleranceOfPric1388pric13880500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1388PRIC1_388\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13880600(
        &mut self,
    ) -> EnblWrProtOfPric1388pric13880600W<PricIo388Spec> {
        EnblWrProtOfPric1388pric13880600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer0(&mut self) -> EnblReadGroup0ofTimer0W<PricIo388Spec> {
        EnblReadGroup0ofTimer0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer0(&mut self) -> EnblReadGroup1ofTimer0W<PricIo388Spec> {
        EnblReadGroup1ofTimer0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer0(&mut self) -> EnblReadGroup2ofTimer0W<PricIo388Spec> {
        EnblReadGroup2ofTimer0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer0(&mut self) -> EnblReadGroup3ofTimer0W<PricIo388Spec> {
        EnblReadGroup3ofTimer0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer0(&mut self) -> EnblReadGroup4ofTimer0W<PricIo388Spec> {
        EnblReadGroup4ofTimer0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of TIMER0"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer0(&mut self) -> EnblReadGroup5ofTimer0W<PricIo388Spec> {
        EnblReadGroup5ofTimer0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13881308(
        &mut self,
    ) -> EnblRstToleranceOfPric1388pric13881308W<PricIo388Spec> {
        EnblRstToleranceOfPric1388pric13881308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1388PRIC1_388\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13881408(
        &mut self,
    ) -> EnblWrProtOfPric1388pric13881408W<PricIo388Spec> {
        EnblWrProtOfPric1388pric13881408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer1(&mut self) -> EnblReadGroup0ofTimer1W<PricIo388Spec> {
        EnblReadGroup0ofTimer1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer1(&mut self) -> EnblReadGroup1ofTimer1W<PricIo388Spec> {
        EnblReadGroup1ofTimer1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer1(&mut self) -> EnblReadGroup2ofTimer1W<PricIo388Spec> {
        EnblReadGroup2ofTimer1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer1(&mut self) -> EnblReadGroup3ofTimer1W<PricIo388Spec> {
        EnblReadGroup3ofTimer1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer1(&mut self) -> EnblReadGroup4ofTimer1W<PricIo388Spec> {
        EnblReadGroup4ofTimer1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of TIMER1"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer1(&mut self) -> EnblReadGroup5ofTimer1W<PricIo388Spec> {
        EnblReadGroup5ofTimer1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13882116(
        &mut self,
    ) -> EnblRstToleranceOfPric1388pric13882116W<PricIo388Spec> {
        EnblRstToleranceOfPric1388pric13882116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1388PRIC1_388\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13882216(
        &mut self,
    ) -> EnblWrProtOfPric1388pric13882216W<PricIo388Spec> {
        EnblWrProtOfPric1388pric13882216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer2(&mut self) -> EnblReadGroup0ofTimer2W<PricIo388Spec> {
        EnblReadGroup0ofTimer2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer2(&mut self) -> EnblReadGroup1ofTimer2W<PricIo388Spec> {
        EnblReadGroup1ofTimer2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer2(&mut self) -> EnblReadGroup2ofTimer2W<PricIo388Spec> {
        EnblReadGroup2ofTimer2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer2(&mut self) -> EnblReadGroup3ofTimer2W<PricIo388Spec> {
        EnblReadGroup3ofTimer2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer2(&mut self) -> EnblReadGroup4ofTimer2W<PricIo388Spec> {
        EnblReadGroup4ofTimer2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of TIMER2"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer2(&mut self) -> EnblReadGroup5ofTimer2W<PricIo388Spec> {
        EnblReadGroup5ofTimer2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1388PRIC1_388\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1388pric13882924(
        &mut self,
    ) -> EnblRstToleranceOfPric1388pric13882924W<PricIo388Spec> {
        EnblRstToleranceOfPric1388pric13882924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1388PRIC1_388\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1388pric13883024(
        &mut self,
    ) -> EnblWrProtOfPric1388pric13883024W<PricIo388Spec> {
        EnblWrProtOfPric1388pric13883024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io388::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io388::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo388Spec;
impl crate::RegisterSpec for PricIo388Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io388::R`](R) reader structure"]
impl crate::Readable for PricIo388Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io388::W`](W) writer structure"]
impl crate::Writable for PricIo388Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO388 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo388Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
