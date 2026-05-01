#[doc = "Register `GPIO9B0` reader"]
pub type R = crate::R<Gpio9b0Spec>;
#[doc = "Register `GPIO9B0` writer"]
pub type W = crate::W<Gpio9b0Spec>;
#[doc = "Field `GPIO160ReadPrivilegeOfMaster` reader - GPIO160 Read Privilege of Master"]
pub type Gpio160readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO160ReadPrivilegeOfMaster` writer - GPIO160 Read Privilege of Master"]
pub type Gpio160readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO161ReadPrivilegeOfMaster` reader - GPIO161 Read Privilege of Master"]
pub type Gpio161readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO161ReadPrivilegeOfMaster` writer - GPIO161 Read Privilege of Master"]
pub type Gpio161readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO162ReadPrivilegeOfMaster` reader - GPIO162 Read Privilege of Master"]
pub type Gpio162readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO162ReadPrivilegeOfMaster` writer - GPIO162 Read Privilege of Master"]
pub type Gpio162readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO163ReadPrivilegeOfMaster` reader - GPIO163 Read Privilege of Master"]
pub type Gpio163readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO163ReadPrivilegeOfMaster` writer - GPIO163 Read Privilege of Master"]
pub type Gpio163readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO160 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio160read_privilege_of_master(&self) -> Gpio160readPrivilegeOfMasterR {
        Gpio160readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO161 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio161read_privilege_of_master(&self) -> Gpio161readPrivilegeOfMasterR {
        Gpio161readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO162 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio162read_privilege_of_master(&self) -> Gpio162readPrivilegeOfMasterR {
        Gpio162readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO163 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio163read_privilege_of_master(&self) -> Gpio163readPrivilegeOfMasterR {
        Gpio163readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO160 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio160read_privilege_of_master(
        &mut self,
    ) -> Gpio160readPrivilegeOfMasterW<Gpio9b0Spec> {
        Gpio160readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO161 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio161read_privilege_of_master(
        &mut self,
    ) -> Gpio161readPrivilegeOfMasterW<Gpio9b0Spec> {
        Gpio161readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO162 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio162read_privilege_of_master(
        &mut self,
    ) -> Gpio162readPrivilegeOfMasterW<Gpio9b0Spec> {
        Gpio162readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO163 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio163read_privilege_of_master(
        &mut self,
    ) -> Gpio163readPrivilegeOfMasterW<Gpio9b0Spec> {
        Gpio163readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9b0Spec;
impl crate::RegisterSpec for Gpio9b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9b0::R`](R) reader structure"]
impl crate::Readable for Gpio9b0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9b0::W`](W) writer structure"]
impl crate::Writable for Gpio9b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9B0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9b0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
