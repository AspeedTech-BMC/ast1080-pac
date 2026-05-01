#[doc = "Register `OTP_REG100` reader"]
pub type R = crate::R<OtpReg100Spec>;
#[doc = "Register `OTP_REG100` writer"]
pub type W = crate::W<OtpReg100Spec>;
#[doc = "Field `REGREGIONROMPATCHREN` reader - REG_REGION_ROMPATCH_REN"]
pub type RegregionrompatchrenR = crate::FieldReader;
#[doc = "Field `REGREGIONROMPATCHREN` writer - REG_REGION_ROMPATCH_REN"]
pub type RegregionrompatchrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONROMPATCHWEN` reader - REG_REGION_ROMPATCH_WEN"]
pub type RegregionrompatchwenR = crate::FieldReader;
#[doc = "Field `REGREGIONROMPATCHWEN` writer - REG_REGION_ROMPATCH_WEN"]
pub type RegregionrompatchwenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONROMPATCHLOCK` reader - REG_REGION_ROMPATCH_LOCK"]
pub type RegregionrompatchlockR = crate::BitReader;
#[doc = "Field `REGREGIONROMPATCHLOCK` writer - REG_REGION_ROMPATCH_LOCK"]
pub type RegregionrompatchlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_ROMPATCH_REN"]
    #[inline(always)]
    pub fn regregionrompatchren(&self) -> RegregionrompatchrenR {
        RegregionrompatchrenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_ROMPATCH_WEN"]
    #[inline(always)]
    pub fn regregionrompatchwen(&self) -> RegregionrompatchwenR {
        RegregionrompatchwenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_ROMPATCH_LOCK"]
    #[inline(always)]
    pub fn regregionrompatchlock(&self) -> RegregionrompatchlockR {
        RegregionrompatchlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_ROMPATCH_REN"]
    #[inline(always)]
    pub fn regregionrompatchren(&mut self) -> RegregionrompatchrenW<OtpReg100Spec> {
        RegregionrompatchrenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_ROMPATCH_WEN"]
    #[inline(always)]
    pub fn regregionrompatchwen(&mut self) -> RegregionrompatchwenW<OtpReg100Spec> {
        RegregionrompatchwenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_ROMPATCH_LOCK"]
    #[inline(always)]
    pub fn regregionrompatchlock(&mut self) -> RegregionrompatchlockW<OtpReg100Spec> {
        RegregionrompatchlockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_ROM\\_PATCH\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg100Spec;
impl crate::RegisterSpec for OtpReg100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg100::R`](R) reader structure"]
impl crate::Readable for OtpReg100Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg100::W`](W) writer structure"]
impl crate::Writable for OtpReg100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG100 to value 0xffff"]
impl crate::Resettable for OtpReg100Spec {
    const RESET_VALUE: u32 = 0xffff;
}
