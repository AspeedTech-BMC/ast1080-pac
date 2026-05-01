#[doc = "Register `OTP_REG130` reader"]
pub type R = crate::R<OtpReg130Spec>;
#[doc = "Register `OTP_REG130` writer"]
pub type W = crate::W<OtpReg130Spec>;
#[doc = "Field `REGREGIONSECURE2REN` reader - REG_REGION_SECURE2_REN"]
pub type Regregionsecure2renR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE2REN` writer - REG_REGION_SECURE2_REN"]
pub type Regregionsecure2renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE2WEN` reader - REG_REGION_SECURE2_WEN"]
pub type Regregionsecure2wenR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE2WEN` writer - REG_REGION_SECURE2_WEN"]
pub type Regregionsecure2wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE2LOCK` reader - REG_REGION_SECURE2_LOCK"]
pub type Regregionsecure2lockR = crate::BitReader;
#[doc = "Field `REGREGIONSECURE2LOCK` writer - REG_REGION_SECURE2_LOCK"]
pub type Regregionsecure2lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_SECURE2_REN"]
    #[inline(always)]
    pub fn regregionsecure2ren(&self) -> Regregionsecure2renR {
        Regregionsecure2renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE2_WEN"]
    #[inline(always)]
    pub fn regregionsecure2wen(&self) -> Regregionsecure2wenR {
        Regregionsecure2wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE2_LOCK"]
    #[inline(always)]
    pub fn regregionsecure2lock(&self) -> Regregionsecure2lockR {
        Regregionsecure2lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_SECURE2_REN"]
    #[inline(always)]
    pub fn regregionsecure2ren(&mut self) -> Regregionsecure2renW<OtpReg130Spec> {
        Regregionsecure2renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE2_WEN"]
    #[inline(always)]
    pub fn regregionsecure2wen(&mut self) -> Regregionsecure2wenW<OtpReg130Spec> {
        Regregionsecure2wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE2_LOCK"]
    #[inline(always)]
    pub fn regregionsecure2lock(&mut self) -> Regregionsecure2lockW<OtpReg130Spec> {
        Regregionsecure2lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_SECURE2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg130::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg130::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg130Spec;
impl crate::RegisterSpec for OtpReg130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg130::R`](R) reader structure"]
impl crate::Readable for OtpReg130Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg130::W`](W) writer structure"]
impl crate::Writable for OtpReg130Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG130 to value 0xffff"]
impl crate::Resettable for OtpReg130Spec {
    const RESET_VALUE: u32 = 0xffff;
}
