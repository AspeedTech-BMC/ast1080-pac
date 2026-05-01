#[doc = "Register `PRIC_IO208` reader"]
pub type R = crate::R<PricIo208Spec>;
#[doc = "Register `PRIC_IO208` writer"]
pub type W = crate::W<PricIo208Spec>;
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
#[doc = "Field `EnblWrGroup0OfAHBC` reader - Enable Write Group #0 of AHBC"]
pub type EnblWrGroup0ofAhbcR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfAHBC` writer - Enable Write Group #0 of AHBC"]
pub type EnblWrGroup0ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfAHBC` reader - Enable Write Group #1 of AHBC"]
pub type EnblWrGroup1ofAhbcR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfAHBC` writer - Enable Write Group #1 of AHBC"]
pub type EnblWrGroup1ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfAHBC` reader - Enable Write Group #2 of AHBC"]
pub type EnblWrGroup2ofAhbcR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfAHBC` writer - Enable Write Group #2 of AHBC"]
pub type EnblWrGroup2ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfAHBC` reader - Enable Write Group #3 of AHBC"]
pub type EnblWrGroup3ofAhbcR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfAHBC` writer - Enable Write Group #3 of AHBC"]
pub type EnblWrGroup3ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfAHBC` reader - Enable Write Group #4 of AHBC"]
pub type EnblWrGroup4ofAhbcR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfAHBC` writer - Enable Write Group #4 of AHBC"]
pub type EnblWrGroup4ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfAHBC` reader - Enable Write Group #5 of AHBC"]
pub type EnblWrGroup5ofAhbcR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfAHBC` writer - Enable Write Group #5 of AHBC"]
pub type EnblWrGroup5ofAhbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1208PRIC1_208\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1208pric12081308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1208pric12081308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1208pric12081308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1208PRIC12081308` reader - Enable Reset Tolerance of PRIC1208PRIC1_208\\[13:08\\]"]
pub type EnblRstToleranceOfPric1208pric12081308R =
    crate::BitReader<EnblRstToleranceOfPric1208pric12081308>;
impl EnblRstToleranceOfPric1208pric12081308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1208pric12081308 {
        match self.bits {
            false => EnblRstToleranceOfPric1208pric12081308::ResetBySrst,
            true => EnblRstToleranceOfPric1208pric12081308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1208pric12081308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1208pric12081308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1208PRIC12081308` writer - Enable Reset Tolerance of PRIC1208PRIC1_208\\[13:08\\]"]
pub type EnblRstToleranceOfPric1208pric12081308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1208pric12081308>;
impl<'a, REG> EnblRstToleranceOfPric1208pric12081308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1208pric12081308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1208pric12081308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1208PRIC12081408` reader - Enable Write Protection of PRIC1208PRIC1_208\\[14:08\\]"]
pub type EnblWrProtOfPric1208pric12081408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1208PRIC12081408` writer - Enable Write Protection of PRIC1208PRIC1_208\\[14:08\\]"]
pub type EnblWrProtOfPric1208pric12081408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfPWM` reader - Enable Write Group #0 of PWM"]
pub type EnblWrGroup0ofPwmR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfPWM` writer - Enable Write Group #0 of PWM"]
pub type EnblWrGroup0ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfPWM` reader - Enable Write Group #1 of PWM"]
pub type EnblWrGroup1ofPwmR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfPWM` writer - Enable Write Group #1 of PWM"]
pub type EnblWrGroup1ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfPWM` reader - Enable Write Group #2 of PWM"]
pub type EnblWrGroup2ofPwmR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfPWM` writer - Enable Write Group #2 of PWM"]
pub type EnblWrGroup2ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfPWM` reader - Enable Write Group #3 of PWM"]
pub type EnblWrGroup3ofPwmR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfPWM` writer - Enable Write Group #3 of PWM"]
pub type EnblWrGroup3ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfPWM` reader - Enable Write Group #4 of PWM"]
pub type EnblWrGroup4ofPwmR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfPWM` writer - Enable Write Group #4 of PWM"]
pub type EnblWrGroup4ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfPWM` reader - Enable Write Group #5 of PWM"]
pub type EnblWrGroup5ofPwmR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfPWM` writer - Enable Write Group #5 of PWM"]
pub type EnblWrGroup5ofPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1208PRIC1_208\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1208pric12082116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1208pric12082116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1208pric12082116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1208PRIC12082116` reader - Enable Reset Tolerance of PRIC1208PRIC1_208\\[21:16\\]"]
pub type EnblRstToleranceOfPric1208pric12082116R =
    crate::BitReader<EnblRstToleranceOfPric1208pric12082116>;
impl EnblRstToleranceOfPric1208pric12082116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1208pric12082116 {
        match self.bits {
            false => EnblRstToleranceOfPric1208pric12082116::ResetBySrst,
            true => EnblRstToleranceOfPric1208pric12082116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1208pric12082116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1208pric12082116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1208PRIC12082116` writer - Enable Reset Tolerance of PRIC1208PRIC1_208\\[21:16\\]"]
pub type EnblRstToleranceOfPric1208pric12082116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1208pric12082116>;
impl<'a, REG> EnblRstToleranceOfPric1208pric12082116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1208pric12082116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1208pric12082116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1208PRIC12082216` reader - Enable Write Protection of PRIC1208PRIC1_208\\[22:16\\]"]
pub type EnblWrProtOfPric1208pric12082216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1208PRIC12082216` writer - Enable Write Protection of PRIC1208PRIC1_208\\[22:16\\]"]
pub type EnblWrProtOfPric1208pric12082216W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - Enable Write Group #0 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ahbc(&self) -> EnblWrGroup0ofAhbcR {
        EnblWrGroup0ofAhbcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ahbc(&self) -> EnblWrGroup1ofAhbcR {
        EnblWrGroup1ofAhbcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ahbc(&self) -> EnblWrGroup2ofAhbcR {
        EnblWrGroup2ofAhbcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ahbc(&self) -> EnblWrGroup3ofAhbcR {
        EnblWrGroup3ofAhbcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ahbc(&self) -> EnblWrGroup4ofAhbcR {
        EnblWrGroup4ofAhbcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ahbc(&self) -> EnblWrGroup5ofAhbcR {
        EnblWrGroup5ofAhbcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1208PRIC1_208\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1208pric12081308(
        &self,
    ) -> EnblRstToleranceOfPric1208pric12081308R {
        EnblRstToleranceOfPric1208pric12081308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1208PRIC1_208\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1208pric12081408(&self) -> EnblWrProtOfPric1208pric12081408R {
        EnblWrProtOfPric1208pric12081408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group0of_pwm(&self) -> EnblWrGroup0ofPwmR {
        EnblWrGroup0ofPwmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group1of_pwm(&self) -> EnblWrGroup1ofPwmR {
        EnblWrGroup1ofPwmR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group2of_pwm(&self) -> EnblWrGroup2ofPwmR {
        EnblWrGroup2ofPwmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group3of_pwm(&self) -> EnblWrGroup3ofPwmR {
        EnblWrGroup3ofPwmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group4of_pwm(&self) -> EnblWrGroup4ofPwmR {
        EnblWrGroup4ofPwmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group5of_pwm(&self) -> EnblWrGroup5ofPwmR {
        EnblWrGroup5ofPwmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1208PRIC1_208\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1208pric12082116(
        &self,
    ) -> EnblRstToleranceOfPric1208pric12082116R {
        EnblRstToleranceOfPric1208pric12082116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1208PRIC1_208\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1208pric12082216(&self) -> EnblWrProtOfPric1208pric12082216R {
        EnblWrProtOfPric1208pric12082216R::new(((self.bits >> 23) & 1) != 0)
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
    pub fn reserved15(&mut self) -> Reserved15W<PricIo208Spec> {
        Reserved15W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<PricIo208Spec> {
        Reserved14W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<PricIo208Spec> {
        Reserved13W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<PricIo208Spec> {
        Reserved12W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<PricIo208Spec> {
        Reserved11W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<PricIo208Spec> {
        Reserved10W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo208Spec> {
        Reserved9W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo208Spec> {
        Reserved8W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ahbc(&mut self) -> EnblWrGroup0ofAhbcW<PricIo208Spec> {
        EnblWrGroup0ofAhbcW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ahbc(&mut self) -> EnblWrGroup1ofAhbcW<PricIo208Spec> {
        EnblWrGroup1ofAhbcW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ahbc(&mut self) -> EnblWrGroup2ofAhbcW<PricIo208Spec> {
        EnblWrGroup2ofAhbcW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ahbc(&mut self) -> EnblWrGroup3ofAhbcW<PricIo208Spec> {
        EnblWrGroup3ofAhbcW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ahbc(&mut self) -> EnblWrGroup4ofAhbcW<PricIo208Spec> {
        EnblWrGroup4ofAhbcW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of AHBC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ahbc(&mut self) -> EnblWrGroup5ofAhbcW<PricIo208Spec> {
        EnblWrGroup5ofAhbcW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1208PRIC1_208\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1208pric12081308(
        &mut self,
    ) -> EnblRstToleranceOfPric1208pric12081308W<PricIo208Spec> {
        EnblRstToleranceOfPric1208pric12081308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1208PRIC1_208\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1208pric12081408(
        &mut self,
    ) -> EnblWrProtOfPric1208pric12081408W<PricIo208Spec> {
        EnblWrProtOfPric1208pric12081408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group0of_pwm(&mut self) -> EnblWrGroup0ofPwmW<PricIo208Spec> {
        EnblWrGroup0ofPwmW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group1of_pwm(&mut self) -> EnblWrGroup1ofPwmW<PricIo208Spec> {
        EnblWrGroup1ofPwmW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group2of_pwm(&mut self) -> EnblWrGroup2ofPwmW<PricIo208Spec> {
        EnblWrGroup2ofPwmW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group3of_pwm(&mut self) -> EnblWrGroup3ofPwmW<PricIo208Spec> {
        EnblWrGroup3ofPwmW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group4of_pwm(&mut self) -> EnblWrGroup4ofPwmW<PricIo208Spec> {
        EnblWrGroup4ofPwmW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of PWM"]
    #[inline(always)]
    pub fn enbl_wr_group5of_pwm(&mut self) -> EnblWrGroup5ofPwmW<PricIo208Spec> {
        EnblWrGroup5ofPwmW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1208PRIC1_208\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1208pric12082116(
        &mut self,
    ) -> EnblRstToleranceOfPric1208pric12082116W<PricIo208Spec> {
        EnblRstToleranceOfPric1208pric12082116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1208PRIC1_208\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1208pric12082216(
        &mut self,
    ) -> EnblWrProtOfPric1208pric12082216W<PricIo208Spec> {
        EnblWrProtOfPric1208pric12082216W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo208Spec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo208Spec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo208Spec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo208Spec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo208Spec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo208Spec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo208Spec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Slave Write Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io208::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io208::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo208Spec;
impl crate::RegisterSpec for PricIo208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io208::R`](R) reader structure"]
impl crate::Readable for PricIo208Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io208::W`](W) writer structure"]
impl crate::Writable for PricIo208Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO208 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo208Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
