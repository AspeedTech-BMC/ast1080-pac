#[doc = "Register `OTP_REG178` reader"]
pub type R = crate::R<OtpReg178Spec>;
#[doc = "Register `OTP_REG178` writer"]
pub type W = crate::W<OtpReg178Spec>;
#[doc = "Field `REGREGIONCALIPTRA3REN` reader - REG_REGION_CALIPTRA3_REN"]
pub type Regregioncaliptra3renR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA3REN` writer - REG_REGION_CALIPTRA3_REN"]
pub type Regregioncaliptra3renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA3WEN` reader - REG_REGION_CALIPTRA3_WEN"]
pub type Regregioncaliptra3wenR = crate::FieldReader;
#[doc = "Field `REGREGIONCALIPTRA3WEN` writer - REG_REGION_CALIPTRA3_WEN"]
pub type Regregioncaliptra3wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONCALIPTRA3LOCK` reader - REG_REGION_CALIPTRA3_LOCK"]
pub type Regregioncaliptra3lockR = crate::BitReader;
#[doc = "Field `REGREGIONCALIPTRA3LOCK` writer - REG_REGION_CALIPTRA3_LOCK"]
pub type Regregioncaliptra3lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA3_REN"]
    #[inline(always)]
    pub fn regregioncaliptra3ren(&self) -> Regregioncaliptra3renR {
        Regregioncaliptra3renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA3_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra3wen(&self) -> Regregioncaliptra3wenR {
        Regregioncaliptra3wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA3_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra3lock(&self) -> Regregioncaliptra3lockR {
        Regregioncaliptra3lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_CALIPTRA3_REN"]
    #[inline(always)]
    pub fn regregioncaliptra3ren(&mut self) -> Regregioncaliptra3renW<OtpReg178Spec> {
        Regregioncaliptra3renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_CALIPTRA3_WEN"]
    #[inline(always)]
    pub fn regregioncaliptra3wen(&mut self) -> Regregioncaliptra3wenW<OtpReg178Spec> {
        Regregioncaliptra3wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_CALIPTRA3_LOCK"]
    #[inline(always)]
    pub fn regregioncaliptra3lock(&mut self) -> Regregioncaliptra3lockW<OtpReg178Spec> {
        Regregioncaliptra3lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_CALIPTRA\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg178Spec;
impl crate::RegisterSpec for OtpReg178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg178::R`](R) reader structure"]
impl crate::Readable for OtpReg178Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg178::W`](W) writer structure"]
impl crate::Writable for OtpReg178Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG178 to value 0xffff"]
impl crate::Resettable for OtpReg178Spec {
    const RESET_VALUE: u32 = 0xffff;
}
