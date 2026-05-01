#[doc = "Register `PRIC_IO308` reader"]
pub type R = crate::R<PricIo308Spec>;
#[doc = "Register `PRIC_IO308` writer"]
pub type W = crate::W<PricIo308Spec>;
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `Reserved15` writer - Reserved"]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `Reserved14` writer - Reserved"]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `Reserved13` writer - Reserved"]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved12` reader - Reserved"]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `Reserved12` writer - Reserved"]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `Reserved11` writer - Reserved"]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `Reserved10` writer - Reserved"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfAHBC` reader - Enable Read Group #0 of AHBC"]
pub type EnblReadGroup0ofAhbcR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfAHBC` writer - Enable Read Group #0 of AHBC"]
pub type EnblReadGroup0ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfAHBC` reader - Enable Read Group #1 of AHBC"]
pub type EnblReadGroup1ofAhbcR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfAHBC` writer - Enable Read Group #1 of AHBC"]
pub type EnblReadGroup1ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfAHBC` reader - Enable Read Group #2 of AHBC"]
pub type EnblReadGroup2ofAhbcR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfAHBC` writer - Enable Read Group #2 of AHBC"]
pub type EnblReadGroup2ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfAHBC` reader - Enable Read Group #3 of AHBC"]
pub type EnblReadGroup3ofAhbcR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfAHBC` writer - Enable Read Group #3 of AHBC"]
pub type EnblReadGroup3ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfAHBC` reader - Enable Read Group #4 of AHBC"]
pub type EnblReadGroup4ofAhbcR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfAHBC` writer - Enable Read Group #4 of AHBC"]
pub type EnblReadGroup4ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfAHBC` reader - Enable Read Group #5 of AHBC"]
pub type EnblReadGroup5ofAhbcR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfAHBC` writer - Enable Read Group #5 of AHBC"]
pub type EnblReadGroup5ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1308PRIC1_308\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1308pric13081308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1308pric13081308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1308pric13081308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1308PRIC13081308` reader - Enable Reset Tolerance of PRIC1308PRIC1_308\\[13:08\\]"]
pub type EnblRstToleranceOfPric1308pric13081308R =
    crate::BitReader<EnblRstToleranceOfPric1308pric13081308>;
impl EnblRstToleranceOfPric1308pric13081308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1308pric13081308 {
        match self.bits {
            false => EnblRstToleranceOfPric1308pric13081308::ResetBySrst,
            true => EnblRstToleranceOfPric1308pric13081308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1308pric13081308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1308pric13081308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1308PRIC13081308` writer - Enable Reset Tolerance of PRIC1308PRIC1_308\\[13:08\\]"]
pub type EnblRstToleranceOfPric1308pric13081308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1308pric13081308>;
impl<'a, REG> EnblRstToleranceOfPric1308pric13081308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1308pric13081308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1308pric13081308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1308PRIC13081408` reader - Enable Write Protection of PRIC1308PRIC1_308\\[14:08\\]"]
pub type EnblWrProtOfPric1308pric13081408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1308PRIC13081408` writer - Enable Write Protection of PRIC1308PRIC1_308\\[14:08\\]"]
pub type EnblWrProtOfPric1308pric13081408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfPWM` reader - Enable Read Group #0 of PWM"]
pub type EnblReadGroup0ofPwmR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfPWM` writer - Enable Read Group #0 of PWM"]
pub type EnblReadGroup0ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfPWM` reader - Enable Read Group #1 of PWM"]
pub type EnblReadGroup1ofPwmR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfPWM` writer - Enable Read Group #1 of PWM"]
pub type EnblReadGroup1ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfPWM` reader - Enable Read Group #2 of PWM"]
pub type EnblReadGroup2ofPwmR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfPWM` writer - Enable Read Group #2 of PWM"]
pub type EnblReadGroup2ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfPWM` reader - Enable Read Group #3 of PWM"]
pub type EnblReadGroup3ofPwmR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfPWM` writer - Enable Read Group #3 of PWM"]
pub type EnblReadGroup3ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfPWM` reader - Enable Read Group #4 of PWM"]
pub type EnblReadGroup4ofPwmR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfPWM` writer - Enable Read Group #4 of PWM"]
pub type EnblReadGroup4ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfPWM` reader - Enable Read Group #5 of PWM"]
pub type EnblReadGroup5ofPwmR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfPWM` writer - Enable Read Group #5 of PWM"]
pub type EnblReadGroup5ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1308PRIC1_308\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1308pric13082116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1308pric13082116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1308pric13082116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1308PRIC13082116` reader - Enable Reset Tolerance of PRIC1308PRIC1_308\\[21:16\\]"]
pub type EnblRstToleranceOfPric1308pric13082116R =
    crate::BitReader<EnblRstToleranceOfPric1308pric13082116>;
impl EnblRstToleranceOfPric1308pric13082116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1308pric13082116 {
        match self.bits {
            false => EnblRstToleranceOfPric1308pric13082116::ResetBySrst,
            true => EnblRstToleranceOfPric1308pric13082116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1308pric13082116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1308pric13082116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1308PRIC13082116` writer - Enable Reset Tolerance of PRIC1308PRIC1_308\\[21:16\\]"]
pub type EnblRstToleranceOfPric1308pric13082116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1308pric13082116>;
impl<'a, REG> EnblRstToleranceOfPric1308pric13082116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1308pric13082116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1308pric13082116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1308PRIC13082216` reader - Enable Write Protection of PRIC1308PRIC1_308\\[22:16\\]"]
pub type EnblWrProtOfPric1308pric13082216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1308PRIC13082216` writer - Enable Write Protection of PRIC1308PRIC1_308\\[22:16\\]"]
pub type EnblWrProtOfPric1308pric13082216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group0of_ahbc(&self) -> EnblReadGroup0ofAhbcR {
        EnblReadGroup0ofAhbcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group1of_ahbc(&self) -> EnblReadGroup1ofAhbcR {
        EnblReadGroup1ofAhbcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group2of_ahbc(&self) -> EnblReadGroup2ofAhbcR {
        EnblReadGroup2ofAhbcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group3of_ahbc(&self) -> EnblReadGroup3ofAhbcR {
        EnblReadGroup3ofAhbcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group4of_ahbc(&self) -> EnblReadGroup4ofAhbcR {
        EnblReadGroup4ofAhbcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group5of_ahbc(&self) -> EnblReadGroup5ofAhbcR {
        EnblReadGroup5ofAhbcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1308PRIC1_308\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1308pric13081308(
        &self,
    ) -> EnblRstToleranceOfPric1308pric13081308R {
        EnblRstToleranceOfPric1308pric13081308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1308PRIC1_308\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1308pric13081408(&self) -> EnblWrProtOfPric1308pric13081408R {
        EnblWrProtOfPric1308pric13081408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group0of_pwm(&self) -> EnblReadGroup0ofPwmR {
        EnblReadGroup0ofPwmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group1of_pwm(&self) -> EnblReadGroup1ofPwmR {
        EnblReadGroup1ofPwmR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group2of_pwm(&self) -> EnblReadGroup2ofPwmR {
        EnblReadGroup2ofPwmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group3of_pwm(&self) -> EnblReadGroup3ofPwmR {
        EnblReadGroup3ofPwmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group4of_pwm(&self) -> EnblReadGroup4ofPwmR {
        EnblReadGroup4ofPwmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group5of_pwm(&self) -> EnblReadGroup5ofPwmR {
        EnblReadGroup5ofPwmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1308PRIC1_308\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1308pric13082116(
        &self,
    ) -> EnblRstToleranceOfPric1308pric13082116R {
        EnblRstToleranceOfPric1308pric13082116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1308PRIC1_308\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1308pric13082216(&self) -> EnblWrProtOfPric1308pric13082216R {
        EnblWrProtOfPric1308pric13082216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&mut self) -> Reserved15W<PricIo308Spec> {
        Reserved15W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<PricIo308Spec> {
        Reserved14W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<PricIo308Spec> {
        Reserved13W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<PricIo308Spec> {
        Reserved12W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<PricIo308Spec> {
        Reserved11W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<PricIo308Spec> {
        Reserved10W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo308Spec> {
        Reserved9W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo308Spec> {
        Reserved8W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group0of_ahbc(&mut self) -> EnblReadGroup0ofAhbcW<PricIo308Spec> {
        EnblReadGroup0ofAhbcW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group1of_ahbc(&mut self) -> EnblReadGroup1ofAhbcW<PricIo308Spec> {
        EnblReadGroup1ofAhbcW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group2of_ahbc(&mut self) -> EnblReadGroup2ofAhbcW<PricIo308Spec> {
        EnblReadGroup2ofAhbcW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group3of_ahbc(&mut self) -> EnblReadGroup3ofAhbcW<PricIo308Spec> {
        EnblReadGroup3ofAhbcW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group4of_ahbc(&mut self) -> EnblReadGroup4ofAhbcW<PricIo308Spec> {
        EnblReadGroup4ofAhbcW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of AHBC"]
    #[inline(always)]
    pub fn enbl_read_group5of_ahbc(&mut self) -> EnblReadGroup5ofAhbcW<PricIo308Spec> {
        EnblReadGroup5ofAhbcW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1308PRIC1_308\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1308pric13081308(
        &mut self,
    ) -> EnblRstToleranceOfPric1308pric13081308W<PricIo308Spec> {
        EnblRstToleranceOfPric1308pric13081308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1308PRIC1_308\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1308pric13081408(
        &mut self,
    ) -> EnblWrProtOfPric1308pric13081408W<PricIo308Spec> {
        EnblWrProtOfPric1308pric13081408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group0of_pwm(&mut self) -> EnblReadGroup0ofPwmW<PricIo308Spec> {
        EnblReadGroup0ofPwmW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group1of_pwm(&mut self) -> EnblReadGroup1ofPwmW<PricIo308Spec> {
        EnblReadGroup1ofPwmW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group2of_pwm(&mut self) -> EnblReadGroup2ofPwmW<PricIo308Spec> {
        EnblReadGroup2ofPwmW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group3of_pwm(&mut self) -> EnblReadGroup3ofPwmW<PricIo308Spec> {
        EnblReadGroup3ofPwmW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group4of_pwm(&mut self) -> EnblReadGroup4ofPwmW<PricIo308Spec> {
        EnblReadGroup4ofPwmW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of PWM"]
    #[inline(always)]
    pub fn enbl_read_group5of_pwm(&mut self) -> EnblReadGroup5ofPwmW<PricIo308Spec> {
        EnblReadGroup5ofPwmW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1308PRIC1_308\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1308pric13082116(
        &mut self,
    ) -> EnblRstToleranceOfPric1308pric13082116W<PricIo308Spec> {
        EnblRstToleranceOfPric1308pric13082116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1308PRIC1_308\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1308pric13082216(
        &mut self,
    ) -> EnblWrProtOfPric1308pric13082216W<PricIo308Spec> {
        EnblWrProtOfPric1308pric13082216W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo308Spec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo308Spec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo308Spec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo308Spec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo308Spec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo308Spec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo308Spec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Slave Read Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io308::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io308::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo308Spec;
impl crate::RegisterSpec for PricIo308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io308::R`](R) reader structure"]
impl crate::Readable for PricIo308Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io308::W`](W) writer structure"]
impl crate::Writable for PricIo308Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO308 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo308Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
