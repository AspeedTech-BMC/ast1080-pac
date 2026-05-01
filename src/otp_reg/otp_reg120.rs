#[doc = "Register `OTP_REG120` reader"]
pub type R = crate::R<OtpReg120Spec>;
#[doc = "Register `OTP_REG120` writer"]
pub type W = crate::W<OtpReg120Spec>;
#[doc = "Field `REGREGIONSECURE0REN` reader - REG_REGION_SECURE0_REN"]
pub type Regregionsecure0renR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE0REN` writer - REG_REGION_SECURE0_REN"]
pub type Regregionsecure0renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE0WEN` reader - REG_REGION_SECURE0_WEN"]
pub type Regregionsecure0wenR = crate::FieldReader;
#[doc = "Field `REGREGIONSECURE0WEN` writer - REG_REGION_SECURE0_WEN"]
pub type Regregionsecure0wenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONSECURE0LOCK` reader - REG_REGION_SECURE0_LOCK"]
pub type Regregionsecure0lockR = crate::BitReader;
#[doc = "Field `REGREGIONSECURE0LOCK` writer - REG_REGION_SECURE0_LOCK"]
pub type Regregionsecure0lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_SECURE0_REN"]
    #[inline(always)]
    pub fn regregionsecure0ren(&self) -> Regregionsecure0renR {
        Regregionsecure0renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE0_WEN"]
    #[inline(always)]
    pub fn regregionsecure0wen(&self) -> Regregionsecure0wenR {
        Regregionsecure0wenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE0_LOCK"]
    #[inline(always)]
    pub fn regregionsecure0lock(&self) -> Regregionsecure0lockR {
        Regregionsecure0lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_SECURE0_REN"]
    #[inline(always)]
    pub fn regregionsecure0ren(&mut self) -> Regregionsecure0renW<OtpReg120Spec> {
        Regregionsecure0renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_SECURE0_WEN"]
    #[inline(always)]
    pub fn regregionsecure0wen(&mut self) -> Regregionsecure0wenW<OtpReg120Spec> {
        Regregionsecure0wenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_SECURE0_LOCK"]
    #[inline(always)]
    pub fn regregionsecure0lock(&mut self) -> Regregionsecure0lockW<OtpReg120Spec> {
        Regregionsecure0lockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_SECURE0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg120Spec;
impl crate::RegisterSpec for OtpReg120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg120::R`](R) reader structure"]
impl crate::Readable for OtpReg120Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg120::W`](W) writer structure"]
impl crate::Writable for OtpReg120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG120 to value 0xffff"]
impl crate::Resettable for OtpReg120Spec {
    const RESET_VALUE: u32 = 0xffff;
}
