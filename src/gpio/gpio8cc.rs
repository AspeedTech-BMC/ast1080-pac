#[doc = "Register `GPIO8CC` reader"]
pub type R = crate::R<Gpio8ccSpec>;
#[doc = "Register `GPIO8CC` writer"]
pub type W = crate::W<Gpio8ccSpec>;
#[doc = "Field `GPIO188WrPrivilegeOfMaster` reader - GPIO188 Write Privilege of Master"]
pub type Gpio188wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO188WrPrivilegeOfMaster` writer - GPIO188 Write Privilege of Master"]
pub type Gpio188wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO189WrPrivilegeOfMaster` reader - GPIO189 Write Privilege of Master"]
pub type Gpio189wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO189WrPrivilegeOfMaster` writer - GPIO189 Write Privilege of Master"]
pub type Gpio189wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO190WrPrivilegeOfMaster` reader - GPIO190 Write Privilege of Master"]
pub type Gpio190wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO190WrPrivilegeOfMaster` writer - GPIO190 Write Privilege of Master"]
pub type Gpio190wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO191WrPrivilegeOfMaster` reader - GPIO191 Write Privilege of Master"]
pub type Gpio191wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO191WrPrivilegeOfMaster` writer - GPIO191 Write Privilege of Master"]
pub type Gpio191wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO188 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio188wr_privilege_of_master(&self) -> Gpio188wrPrivilegeOfMasterR {
        Gpio188wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO189 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio189wr_privilege_of_master(&self) -> Gpio189wrPrivilegeOfMasterR {
        Gpio189wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO190 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio190wr_privilege_of_master(&self) -> Gpio190wrPrivilegeOfMasterR {
        Gpio190wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO191 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio191wr_privilege_of_master(&self) -> Gpio191wrPrivilegeOfMasterR {
        Gpio191wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO188 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio188wr_privilege_of_master(&mut self) -> Gpio188wrPrivilegeOfMasterW<Gpio8ccSpec> {
        Gpio188wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO189 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio189wr_privilege_of_master(&mut self) -> Gpio189wrPrivilegeOfMasterW<Gpio8ccSpec> {
        Gpio189wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO190 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio190wr_privilege_of_master(&mut self) -> Gpio190wrPrivilegeOfMasterW<Gpio8ccSpec> {
        Gpio190wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO191 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio191wr_privilege_of_master(&mut self) -> Gpio191wrPrivilegeOfMasterW<Gpio8ccSpec> {
        Gpio191wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8ccSpec;
impl crate::RegisterSpec for Gpio8ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8cc::R`](R) reader structure"]
impl crate::Readable for Gpio8ccSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio8cc::W`](W) writer structure"]
impl crate::Writable for Gpio8ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8CC to value 0xffff_ffff"]
impl crate::Resettable for Gpio8ccSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
