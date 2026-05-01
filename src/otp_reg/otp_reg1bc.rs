#[doc = "Register `OTP_REG1BC` reader"]
pub type R = crate::R<OtpReg1bcSpec>;
#[doc = "Register `OTP_REG1BC` writer"]
pub type W = crate::W<OtpReg1bcSpec>;
#[doc = "Field `REGMASTERRID4` reader - REG_MASTER_RID4"]
pub type Regmasterrid4R = crate::FieldReader;
#[doc = "Field `REGMASTERRID4` writer - REG_MASTER_RID4"]
pub type Regmasterrid4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERRID5` reader - REG_MASTER_RID5"]
pub type Regmasterrid5R = crate::FieldReader;
#[doc = "Field `REGMASTERRID5` writer - REG_MASTER_RID5"]
pub type Regmasterrid5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERRLOCK` reader - REG_MASTER_RLOCK"]
pub type RegmasterrlockR = crate::BitReader;
#[doc = "Field `REGMASTERRLOCK` writer - REG_MASTER_RLOCK"]
pub type RegmasterrlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_MASTER_RID4"]
    #[inline(always)]
    pub fn regmasterrid4(&self) -> Regmasterrid4R {
        Regmasterrid4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_MASTER_RID5"]
    #[inline(always)]
    pub fn regmasterrid5(&self) -> Regmasterrid5R {
        Regmasterrid5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_MASTER_RLOCK"]
    #[inline(always)]
    pub fn regmasterrlock(&self) -> RegmasterrlockR {
        RegmasterrlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_MASTER_RID4"]
    #[inline(always)]
    pub fn regmasterrid4(&mut self) -> Regmasterrid4W<OtpReg1bcSpec> {
        Regmasterrid4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_MASTER_RID5"]
    #[inline(always)]
    pub fn regmasterrid5(&mut self) -> Regmasterrid5W<OtpReg1bcSpec> {
        Regmasterrid5W::new(self, 8)
    }
    #[doc = "Bit 31 - REG_MASTER_RLOCK"]
    #[inline(always)]
    pub fn regmasterrlock(&mut self) -> RegmasterrlockW<OtpReg1bcSpec> {
        RegmasterrlockW::new(self, 31)
    }
}
#[doc = "OTP\\_R\\_MASTER\\_ID\\_EXT\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1bcSpec;
impl crate::RegisterSpec for OtpReg1bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1bc::R`](R) reader structure"]
impl crate::Readable for OtpReg1bcSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1bc::W`](W) writer structure"]
impl crate::Writable for OtpReg1bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1BC to value 0xffff"]
impl crate::Resettable for OtpReg1bcSpec {
    const RESET_VALUE: u32 = 0xffff;
}
