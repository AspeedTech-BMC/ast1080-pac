#[doc = "Register `GPIO890` reader"]
pub type R = crate::R<Gpio890Spec>;
#[doc = "Register `GPIO890` writer"]
pub type W = crate::W<Gpio890Spec>;
#[doc = "Field `GPIO128WrPrivilegeOfMaster` reader - GPIO128 Write Privilege of Master"]
pub type Gpio128wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO128WrPrivilegeOfMaster` writer - GPIO128 Write Privilege of Master"]
pub type Gpio128wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO129WrPrivilegeOfMaster` reader - GPIO129 Write Privilege of Master"]
pub type Gpio129wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO129WrPrivilegeOfMaster` writer - GPIO129 Write Privilege of Master"]
pub type Gpio129wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO130WrPrivilegeOfMaster` reader - GPIO130 Write Privilege of Master"]
pub type Gpio130wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO130WrPrivilegeOfMaster` writer - GPIO130 Write Privilege of Master"]
pub type Gpio130wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO131WrPrivilegeOfMaster` reader - GPIO131 Write Privilege of Master"]
pub type Gpio131wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO131WrPrivilegeOfMaster` writer - GPIO131 Write Privilege of Master"]
pub type Gpio131wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO128 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio128wr_privilege_of_master(&self) -> Gpio128wrPrivilegeOfMasterR {
        Gpio128wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO129 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio129wr_privilege_of_master(&self) -> Gpio129wrPrivilegeOfMasterR {
        Gpio129wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO130 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio130wr_privilege_of_master(&self) -> Gpio130wrPrivilegeOfMasterR {
        Gpio130wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO131 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio131wr_privilege_of_master(&self) -> Gpio131wrPrivilegeOfMasterR {
        Gpio131wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO128 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio128wr_privilege_of_master(&mut self) -> Gpio128wrPrivilegeOfMasterW<Gpio890Spec> {
        Gpio128wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO129 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio129wr_privilege_of_master(&mut self) -> Gpio129wrPrivilegeOfMasterW<Gpio890Spec> {
        Gpio129wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO130 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio130wr_privilege_of_master(&mut self) -> Gpio130wrPrivilegeOfMasterW<Gpio890Spec> {
        Gpio130wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO131 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio131wr_privilege_of_master(&mut self) -> Gpio131wrPrivilegeOfMasterW<Gpio890Spec> {
        Gpio131wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio890::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio890::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio890Spec;
impl crate::RegisterSpec for Gpio890Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio890::R`](R) reader structure"]
impl crate::Readable for Gpio890Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio890::W`](W) writer structure"]
impl crate::Writable for Gpio890Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO890 to value 0xffff_ffff"]
impl crate::Resettable for Gpio890Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
