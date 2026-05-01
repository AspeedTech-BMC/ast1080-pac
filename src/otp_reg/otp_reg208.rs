#[doc = "Register `OTP_REG208` reader"]
pub type R = crate::R<OtpReg208Spec>;
#[doc = "Register `OTP_REG208` writer"]
pub type W = crate::W<OtpReg208Spec>;
#[doc = "Field `REGINTRMASTERID` reader - REG_INTR_MASTER_ID"]
pub type RegintrmasteridR = crate::FieldReader;
#[doc = "Field `REGINTRMASTERID` writer - REG_INTR_MASTER_ID"]
pub type RegintrmasteridW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGINTRMASTERLOCK` reader - REG_INTR_MASTER_LOCK"]
pub type RegintrmasterlockR = crate::BitReader;
#[doc = "Field `REGINTRMASTERLOCK` writer - REG_INTR_MASTER_LOCK"]
pub type RegintrmasterlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_INTR_MASTER_ID"]
    #[inline(always)]
    pub fn regintrmasterid(&self) -> RegintrmasteridR {
        RegintrmasteridR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_INTR_MASTER_LOCK"]
    #[inline(always)]
    pub fn regintrmasterlock(&self) -> RegintrmasterlockR {
        RegintrmasterlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_INTR_MASTER_ID"]
    #[inline(always)]
    pub fn regintrmasterid(&mut self) -> RegintrmasteridW<OtpReg208Spec> {
        RegintrmasteridW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_INTR_MASTER_LOCK"]
    #[inline(always)]
    pub fn regintrmasterlock(&mut self) -> RegintrmasterlockW<OtpReg208Spec> {
        RegintrmasterlockW::new(self, 31)
    }
}
#[doc = "OTP\\_INTR\\_MID\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg208::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg208::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg208Spec;
impl crate::RegisterSpec for OtpReg208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg208::R`](R) reader structure"]
impl crate::Readable for OtpReg208Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg208::W`](W) writer structure"]
impl crate::Writable for OtpReg208Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG208 to value 0xff"]
impl crate::Resettable for OtpReg208Spec {
    const RESET_VALUE: u32 = 0xff;
}
